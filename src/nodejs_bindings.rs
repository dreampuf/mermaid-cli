use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::MermaidIt;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Node.js wrapper for MermaidIt
#[napi]
pub struct NodeMermaidRenderer {
    inner: Arc<Mutex<MermaidIt>>,
}

#[napi]
impl NodeMermaidRenderer {
    /// Create a new MermaidRenderer instance
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        let renderer = MermaidIt::new()
            .map_err(|e| Error::from_reason(format!("Failed to create renderer: {}", e)))?;
        
        Ok(Self {
            inner: Arc::new(Mutex::new(renderer)),
        })
    }
    
    /// Set custom Mermaid.js content
    #[napi]
    pub async fn set_custom_mermaid(&self, js_content: String) -> Result<()> {
        let mut renderer = self.inner.lock().await;
        renderer.set_custom_mermaid(js_content);
        Ok(())
    }
    
    /// Render diagram to SVG
    #[napi]
    pub async fn render_svg(
        &self,
        diagram_code: String,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f64>,
    ) -> Result<String> {
        let mut renderer = self.inner.lock().await;
        
        let svg = renderer.render_svg(
            &diagram_code,
            width.unwrap_or(800),
            height.unwrap_or(600),
            &background.unwrap_or_else(|| "white".to_string()),
            &theme.unwrap_or_else(|| "default".to_string()),
            scale.unwrap_or(1.0) as f32,
        )
        .await
        .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
        
        Ok(svg)
    }
    
    /// Render diagram to PNG (returns Buffer)
    #[napi]
    pub async fn render_png(
        &self,
        diagram_code: String,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f64>,
    ) -> Result<Buffer> {
        let mut renderer = self.inner.lock().await;
        
        let png_data = renderer.render_png(
            &diagram_code,
            width.unwrap_or(800),
            height.unwrap_or(600),
            &background.unwrap_or_else(|| "white".to_string()),
            &theme.unwrap_or_else(|| "default".to_string()),
            scale.unwrap_or(1.0) as f32,
        )
        .await
        .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
        
        Ok(Buffer::from(png_data))
    }
    
    /// Render diagram to JPEG (returns Buffer)
    #[napi]
    pub async fn render_jpg(
        &self,
        diagram_code: String,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f64>,
        quality: Option<u32>,
    ) -> Result<Buffer> {
        let mut renderer = self.inner.lock().await;
        
        let jpg_data = renderer.render_jpg(
            &diagram_code,
            width.unwrap_or(800),
            height.unwrap_or(600),
            &background.unwrap_or_else(|| "white".to_string()),
            &theme.unwrap_or_else(|| "default".to_string()),
            scale.unwrap_or(1.0) as f32,
            quality.unwrap_or(90) as u8,
        )
        .await
        .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
        
        Ok(Buffer::from(jpg_data))
    }
    
    /// Render diagram to WebP (returns Buffer)
    #[napi]
    pub async fn render_webp(
        &self,
        diagram_code: String,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f64>,
        quality: Option<f64>,
    ) -> Result<Buffer> {
        let mut renderer = self.inner.lock().await;
        
        let webp_data = renderer.render_webp(
            &diagram_code,
            width.unwrap_or(800),
            height.unwrap_or(600),
            &background.unwrap_or_else(|| "white".to_string()),
            &theme.unwrap_or_else(|| "default".to_string()),
            scale.unwrap_or(1.0) as f32,
            quality.unwrap_or(90.0) as f32,
        )
        .await
        .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
        
        Ok(Buffer::from(webp_data))
    }
    
    /// Render diagram to file
    #[napi]
    pub async fn render_to_file(
        &self,
        diagram_code: String,
        output_path: String,
        format: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f64>,
        quality: Option<u32>,
    ) -> Result<()> {
        let mut renderer = self.inner.lock().await;
        let fmt = format.unwrap_or_else(|| "svg".to_string());
        
        let data = match fmt.as_str() {
            "svg" => {
                let svg = renderer.render_svg(
                    &diagram_code,
                    width.unwrap_or(800),
                    height.unwrap_or(600),
                    &background.unwrap_or_else(|| "white".to_string()),
                    &theme.unwrap_or_else(|| "default".to_string()),
                    scale.unwrap_or(1.0) as f32,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
                svg.into_bytes()
            },
            "png" => {
                renderer.render_png(
                    &diagram_code,
                    width.unwrap_or(800),
                    height.unwrap_or(600),
                    &background.unwrap_or_else(|| "white".to_string()),
                    &theme.unwrap_or_else(|| "default".to_string()),
                    scale.unwrap_or(1.0) as f32,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?
            },
            "jpg" | "jpeg" => {
                renderer.render_jpg(
                    &diagram_code,
                    width.unwrap_or(800),
                    height.unwrap_or(600),
                    &background.unwrap_or_else(|| "white".to_string()),
                    &theme.unwrap_or_else(|| "default".to_string()),
                    scale.unwrap_or(1.0) as f32,
                    quality.unwrap_or(90) as u8,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?
            },
            "webp" => {
                renderer.render_webp(
                    &diagram_code,
                    width.unwrap_or(800),
                    height.unwrap_or(600),
                    &background.unwrap_or_else(|| "white".to_string()),
                    &theme.unwrap_or_else(|| "default".to_string()),
                    scale.unwrap_or(1.0) as f32,
                    quality.unwrap_or(90) as f32,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?
            },
            _ => return Err(Error::from_reason(format!("Unsupported format: {}", fmt))),
        };
        
        std::fs::write(&output_path, data)
            .map_err(|e| Error::from_reason(format!("Failed to write file: {}", e)))?;
        
        Ok(())
    }
    
    /// Render diagram to base64 data URL
    #[napi]
    pub async fn render_data_url(
        &self,
        diagram_code: String,
        format: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f64>,
        quality: Option<u32>,
    ) -> Result<String> {
        let mut renderer = self.inner.lock().await;
        let fmt = format.unwrap_or_else(|| "svg".to_string());
        
        match fmt.as_str() {
            "svg" => {
                let svg = renderer.render_svg(
                    &diagram_code,
                    width.unwrap_or(800),
                    height.unwrap_or(600),
                    &background.unwrap_or_else(|| "white".to_string()),
                    &theme.unwrap_or_else(|| "default".to_string()),
                    scale.unwrap_or(1.0) as f32,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
                let encoded = base64::encode(svg.as_bytes());
                Ok(format!("data:image/svg+xml;base64,{}", encoded))
            },
            "png" => {
                let png_data = renderer.render_png(
                    &diagram_code,
                    width.unwrap_or(800),
                    height.unwrap_or(600),
                    &background.unwrap_or_else(|| "white".to_string()),
                    &theme.unwrap_or_else(|| "default".to_string()),
                    scale.unwrap_or(1.0) as f32,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
                let encoded = base64::encode(&png_data);
                Ok(format!("data:image/png;base64,{}", encoded))
            },
            "jpg" | "jpeg" => {
                let jpg_data = renderer.render_jpg(
                    &diagram_code,
                    width.unwrap_or(800),
                    height.unwrap_or(600),
                    &background.unwrap_or_else(|| "white".to_string()),
                    &theme.unwrap_or_else(|| "default".to_string()),
                    scale.unwrap_or(1.0) as f32,
                    quality.unwrap_or(90) as u8,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
                let encoded = base64::encode(&jpg_data);
                Ok(format!("data:image/jpeg;base64,{}", encoded))
            },
            "webp" => {
                let webp_data = renderer.render_webp(
                    &diagram_code,
                    width.unwrap_or(800),
                    height.unwrap_or(600),
                    &background.unwrap_or_else(|| "white".to_string()),
                    &theme.unwrap_or_else(|| "default".to_string()),
                    scale.unwrap_or(1.0) as f32,
                    quality.unwrap_or(90) as f32,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Render failed: {}", e)))?;
                let encoded = base64::encode(&webp_data);
                Ok(format!("data:image/webp;base64,{}", encoded))
            },
            _ => Err(Error::from_reason(format!("Unsupported format: {}", fmt)))
        }
    }
}