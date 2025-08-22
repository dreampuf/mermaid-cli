use crate::{MermaidIt, OutputFormat, RenderOptions as CoreRenderOptions};
use std::sync::{Arc, Mutex};
use std::fs;

// Include the UDL file
uniffi::include_scaffolding!("mermaid_it");

#[derive(Debug, thiserror::Error)]
pub enum MermaidError {
    #[error("Render error: {message}")]
    RenderError { message: String },
    
    #[error("Invalid format: {format}")]
    InvalidFormat { format: String },
    
    #[error("IO error: {message}")]
    IoError { message: String },
}

impl From<anyhow::Error> for MermaidError {
    fn from(err: anyhow::Error) -> Self {
        MermaidError::RenderError {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for MermaidError {
    fn from(err: std::io::Error) -> Self {
        MermaidError::IoError {
            message: err.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub background: String,
    pub theme: String,
    pub scale: f32,
    pub quality: u8,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            format: "svg".to_string(),
            width: 800,
            height: 600,
            background: "white".to_string(),
            theme: "default".to_string(),
            scale: 1.0,
            quality: 90,
        }
    }
}

impl From<RenderOptions> for CoreRenderOptions {
    fn from(opts: RenderOptions) -> Self {
        CoreRenderOptions {
            width: opts.width,
            height: opts.height,
            background: opts.background,
            theme: opts.theme,
            scale: opts.scale,
            quality: opts.quality,
        }
    }
}

pub struct MermaidRenderer {
    inner: Arc<Mutex<MermaidIt>>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl MermaidRenderer {
    pub fn new() -> Result<Self, MermaidError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| MermaidError::RenderError {
                message: format!("Failed to create runtime: {}", e),
            })?;
        
        let renderer = MermaidIt::new()?;
        
        Ok(Self {
            inner: Arc::new(Mutex::new(renderer)),
            runtime: Arc::new(runtime),
        })
    }
    
    pub fn render(&self, diagram_code: String, options: RenderOptions) -> Result<Vec<u8>, MermaidError> {
        let output_format = OutputFormat::from_str(&options.format)
            .ok_or_else(|| MermaidError::InvalidFormat {
                format: options.format.clone(),
            })?;
        
        let core_options: CoreRenderOptions = options.into();
        let inner = self.inner.clone();
        
        self.runtime.block_on(async move {
            let mut renderer = inner.lock().unwrap();
            renderer.render(&diagram_code, output_format, &core_options)
                .await
                .map_err(|e| MermaidError::RenderError {
                    message: e.to_string(),
                })
        })
    }
    
    pub fn render_to_string(&self, diagram_code: String, options: RenderOptions) -> Result<String, MermaidError> {
        let data = self.render(diagram_code, options)?;
        String::from_utf8(data)
            .map_err(|e| MermaidError::RenderError {
                message: format!("Invalid UTF-8: {}", e),
            })
    }
    
    pub fn render_to_file(&self, diagram_code: String, output_path: String, options: RenderOptions) -> Result<(), MermaidError> {
        let data = self.render(diagram_code, options)?;
        fs::write(&output_path, data)?;
        Ok(())
    }
    
    pub fn set_custom_mermaid(&self, js_content: String) {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut renderer = inner.lock().unwrap();
            renderer.set_custom_mermaid(js_content);
        });
    }
}

pub fn create_renderer() -> Result<Arc<MermaidRenderer>, MermaidError> {
    Ok(Arc::new(MermaidRenderer::new()?))
}