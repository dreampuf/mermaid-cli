use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys;
use crate::{MermaidIt, OutputFormat, RenderOptions};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_wasm_bindgen;

// Configure console error panic hook for better error messages in the browser
#[cfg(feature = "wasm")]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// WASM wrapper for MermaidIt
#[wasm_bindgen]
pub struct WasmMermaidRenderer {
    inner: Arc<Mutex<MermaidIt>>,
}

#[wasm_bindgen]
impl WasmMermaidRenderer {
    /// Create a new MermaidRenderer instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmMermaidRenderer, JsValue> {
        set_panic_hook();
        
        let renderer = MermaidIt::new()
            .map_err(|e| JsValue::from_str(&format!("Failed to create renderer: {}", e)))?;
        
        Ok(Self {
            inner: Arc::new(Mutex::new(renderer)),
        })
    }
    
    /// Set custom Mermaid.js content
    #[wasm_bindgen(js_name = setCustomMermaid)]
    pub async fn set_custom_mermaid(&self, js_content: String) -> Result<(), JsValue> {
        let mut renderer = self.inner.lock().await;
        renderer.set_custom_mermaid(js_content);
        Ok(())
    }
    
    /// Render diagram to specified format
    /// Options should be a JavaScript object with: format, width, height, background, theme, scale, quality
    #[wasm_bindgen(js_name = render)]
    pub async fn render(
        &self,
        diagram_code: String,
        options: JsValue,
    ) -> Result<JsValue, JsValue> {
        // Parse options from JavaScript object
        let opts: RenderOptions = if options.is_undefined() || options.is_null() {
            RenderOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options)
                .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
        };
        
        // Get format from options or default to SVG
        let format_str = js_sys::Reflect::get(&options, &JsValue::from_str("format"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "svg".to_string());
        
        let output_format = OutputFormat::from_str(&format_str)
            .ok_or_else(|| JsValue::from_str(&format!("Unsupported format: {}", format_str)))?;
        
        let mut renderer = self.inner.lock().await;
        
        let data = renderer.render(&diagram_code, output_format, &opts)
            .await
            .map_err(|e| JsValue::from_str(&format!("Render failed: {}", e)))?;
        
        // Return string for SVG, Uint8Array for binary formats
        match output_format {
            OutputFormat::Svg => {
                let svg_str = String::from_utf8(data)
                    .map_err(|e| JsValue::from_str(&format!("Invalid UTF-8: {}", e)))?;
                Ok(JsValue::from_str(&svg_str))
            },
            _ => {
                Ok(js_sys::Uint8Array::from(&data[..]).into())
            }
        }
    }
    
    /// Render diagram to a base64 data URL
    #[wasm_bindgen(js_name = renderDataUrl)]
    pub async fn render_data_url(
        &self,
        diagram_code: String,
        options: JsValue,
    ) -> Result<String, JsValue> {
        // Parse options
        let opts: RenderOptions = if options.is_undefined() || options.is_null() {
            RenderOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options.clone())
                .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
        };
        
        // Get format from options
        let format_str = js_sys::Reflect::get(&options, &JsValue::from_str("format"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "svg".to_string());
        
        let output_format = OutputFormat::from_str(&format_str)
            .ok_or_else(|| JsValue::from_str(&format!("Unsupported format: {}", format_str)))?;
        
        let mut renderer = self.inner.lock().await;
        
        let data = renderer.render(&diagram_code, output_format, &opts)
            .await
            .map_err(|e| JsValue::from_str(&format!("Render failed: {}", e)))?;
        
        let encoded = base64::encode(&data);
        
        let mime_type = match output_format {
            OutputFormat::Svg => "image/svg+xml",
            OutputFormat::Png => "image/png",
            OutputFormat::Jpg | OutputFormat::Jpeg => "image/jpeg",
            OutputFormat::Webp => "image/webp",
            OutputFormat::Gif => "image/gif",
        };
        
        Ok(format!("data:{};base64,{}", mime_type, encoded))
    }
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    set_panic_hook();
}