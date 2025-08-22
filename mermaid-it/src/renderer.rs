use anyhow::{Result, Context};
use deno_core::{JsRuntime, RuntimeOptions, Extension, op2};
use serde::Serialize;

// Include the embedded Mermaid.js
include!(concat!(env!("OUT_DIR"), "/mermaid_const.rs"));

#[derive(Debug, Clone, Serialize)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub background: String,
    pub theme: String,
    pub scale: f32,
}

pub struct MermaidRenderer {
    runtime: JsRuntime,
    custom_mermaid: Option<String>,
}

impl MermaidRenderer {
    pub fn new() -> Result<Self> {
        // Create the JavaScript runtime
        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![mermaid_extension::init_ops()],
            ..Default::default()
        });
        
        // Initialize the runtime with basic setup
        runtime.execute_script(
            "[mermaid_init]",
            include_str!("js/init.js")
        )?;
        
        Ok(Self {
            runtime,
            custom_mermaid: None,
        })
    }
    
    pub fn set_custom_mermaid(&mut self, js_content: String) {
        self.custom_mermaid = Some(js_content);
    }
    
    pub async fn render(&mut self, diagram_code: &str, config: RenderConfig) -> Result<String> {
        // First, setup DOM environment BEFORE loading Mermaid.js
        let setup_code = format!(
            r#"
            // Add essential polyfills
            globalThis.structuredClone = globalThis.structuredClone || function(obj) {{
                return JSON.parse(JSON.stringify(obj));
            }};
            
            // Ensure Error object exists with stackTraceLimit
            if (typeof globalThis.Error === 'undefined') {{
                globalThis.Error = function(message) {{
                    this.message = message;
                    this.name = 'Error';
                }};
            }}
            globalThis.Error.stackTraceLimit = 10;
            
            // Setup DOM-like environment for Mermaid
            if (typeof document === 'undefined') {{
                globalThis.document = {{
                    createElement: function(tag) {{
                        return {{
                            tagName: tag.toUpperCase(),
                            style: {{}},
                            setAttribute: function(name, value) {{
                                this[name] = value;
                            }},
                            appendChild: function(child) {{
                                return child;
                            }},
                            removeChild: function(child) {{
                                return child;
                            }},
                            innerHTML: '',
                            textContent: '',
                            querySelector: function() {{ return null; }},
                            querySelectorAll: function() {{ return []; }}
                        }};
                    }},
                    createElementNS: function(ns, tag) {{
                        return this.createElement(tag);
                    }},
                    createTextNode: function(text) {{
                        return {{ nodeValue: text, nodeType: 3 }};
                    }},
                    body: {{
                        appendChild: function(child) {{ return child; }},
                        removeChild: function(child) {{ return child; }}
                    }},
                    querySelector: function() {{ return null; }},
                    querySelectorAll: function() {{ return []; }},
                    getElementById: function() {{ return null; }},
                    addEventListener: function() {{}},
                    removeEventListener: function() {{}},
                    createEvent: function() {{
                        return {{
                            initEvent: function() {{}},
                            preventDefault: function() {{}}
                        }};
                    }},
                    dispatchEvent: function() {{ return true; }}
                }};
                
                globalThis.window = {{
                    document: globalThis.document,
                    location: {{ href: 'http://localhost' }},
                    navigator: {{ userAgent: 'mermaid-it' }},
                    addEventListener: function() {{}},
                    removeEventListener: function() {{}},
                    dispatchEvent: function() {{ return true; }},
                    getComputedStyle: function() {{ 
                        return {{
                            getPropertyValue: function() {{ return ''; }}
                        }};
                    }},
                    matchMedia: function() {{
                        return {{
                            matches: false,
                            addListener: function() {{}},
                            removeListener: function() {{}}
                        }};
                    }},
                    innerWidth: 1024,
                    innerHeight: 768,
                    devicePixelRatio: 1
                }};
            }}
            "#
        );
        
        self.runtime.execute_script(
            "[mermaid_setup]",
            setup_code
        ).context("Failed to setup Mermaid environment")?;
        
        // Now load Mermaid.js after DOM is set up
        let mermaid_js = if let Some(custom) = &self.custom_mermaid {
            custom.clone()
        } else {
            MERMAID_JS.to_string()
        };
        
        // Wrap Mermaid.js in a try-catch to handle browser-specific issues
        let wrapped_mermaid = format!(
            r#"
            (function() {{
                try {{
                    {}
                }} catch (e) {{
                    console.warn('Some Mermaid features may not work:', e.message);
                }}
            }})();
            "#,
            mermaid_js
        );
        
        // Execute wrapped Mermaid.js
        self.runtime.execute_script(
            "[mermaid_lib]",
            wrapped_mermaid
        ).context("Failed to load Mermaid.js")?;
        
        // Initialize Mermaid with configuration
        let init_code = format!(
            r#"
            // Initialize Mermaid with configuration
            if (typeof mermaid !== 'undefined') {{
                mermaid.initialize({{
                    startOnLoad: false,
                    theme: '{}',
                    themeVariables: {{
                        primaryColor: '#fff',
                        primaryTextColor: '#000',
                        primaryBorderColor: '#000',
                        lineColor: '#000',
                        background: '{}'
                    }},
                    flowchart: {{
                        useMaxWidth: true,
                        htmlLabels: true
                    }},
                    securityLevel: 'loose'
                }});
            }}
            "#,
            config.theme,
            config.background
        );
        
        self.runtime.execute_script(
            "[mermaid_init_config]",
            init_code
        ).context("Failed to initialize Mermaid")?;
        
        // Render the diagram
        let render_code = format!(
            r#"
            (async function() {{
                try {{
                    const diagramCode = `{}`;
                    
                    // Create a container element
                    const container = document.createElement('div');
                    container.id = 'mermaid-container';
                    container.innerHTML = diagramCode;
                    
                    // Render using Mermaid
                    const {{ svg }} = await mermaid.render('mermaid-diagram', diagramCode);
                    
                    // Return the SVG with proper dimensions
                    const svgWithDimensions = svg
                        .replace(/<svg/, `<svg width="{}" height="{}" viewBox="0 0 {} {}"`);
                    
                    return svgWithDimensions;
                }} catch (error) {{
                    throw new Error('Failed to render diagram: ' + error.message);
                }}
            }})();
            "#,
            diagram_code.replace('`', r"\`").replace('$', r"\$"),
            config.width,
            config.height,
            config.width,
            config.height
        );
        
        let promise = self.runtime.execute_script(
            "[mermaid_render]",
            render_code
        ).context("Failed to execute render script")?;
        
        // Run the event loop to resolve the promise
        let svg_result = self.runtime.resolve(promise).await?;
        
        // Convert the result to string
        let scope = &mut self.runtime.handle_scope();
        let local = deno_core::v8::Local::new(scope, svg_result);
        let svg_string = local.to_rust_string_lossy(scope);
        
        Ok(svg_string)
    }
}

// Extension for custom ops if needed
mod mermaid_extension {
    use super::*;
    use deno_core::error::AnyError;
    
    #[op2(fast)]
    #[string]
    fn op_log(#[string] msg: String) -> Result<(), AnyError> {
        println!("[Mermaid]: {}", msg);
        Ok(())
    }
    
    deno_core::extension!(
        mermaid_ext,
        ops = [op_log],
    );
    
    pub fn init_ops() -> Extension {
        mermaid_ext::init_ops_and_esm()
    }
}