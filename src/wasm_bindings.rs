use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys;
use crate::MermaidIt;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    
    /// Render diagram to SVG
    #[wasm_bindgen(js_name = renderSvg)]
    pub async fn render_svg(
        &self,
        diagram_code: String,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f32>,
    ) -> Result<String, JsValue> {
        let mut renderer = self.inner.lock().await;
        
        let svg = renderer.render_svg(
            &diagram_code,
            width.unwrap_or(800),
            height.unwrap_or(600),
            &background.unwrap_or_else(|| "white".to_string()),
            &theme.unwrap_or_else(|| "default".to_string()),
            scale.unwrap_or(1.0),
        )
        .await
        .map_err(|e| JsValue::from_str(&format!("Render failed: {}", e)))?;
        
        Ok(svg)
    }
    
    /// Render diagram to PNG (returns Uint8Array)
    #[wasm_bindgen(js_name = renderPng)]
    pub async fn render_png(
        &self,
        diagram_code: String,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f32>,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let mut renderer = self.inner.lock().await;
        
        let png_data = renderer.render_png(
            &diagram_code,
            width.unwrap_or(800),
            height.unwrap_or(600),
            &background.unwrap_or_else(|| "white".to_string()),
            &theme.unwrap_or_else(|| "default".to_string()),
            scale.unwrap_or(1.0),
        )
        .await
        .map_err(|e| JsValue::from_str(&format!("Render failed: {}", e)))?;
        
        Ok(js_sys::Uint8Array::from(&png_data[..]))
    }
    
    /// Render diagram to JPEG (returns Uint8Array)
    #[wasm_bindgen(js_name = renderJpg)]
    pub async fn render_jpg(
        &self,
        diagram_code: String,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f32>,
        quality: Option<u8>,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let mut renderer = self.inner.lock().await;
        
        let jpg_data = renderer.render_jpg(
            &diagram_code,
            width.unwrap_or(800),
            height.unwrap_or(600),
            &background.unwrap_or_else(|| "white".to_string()),
            &theme.unwrap_or_else(|| "default".to_string()),
            scale.unwrap_or(1.0),
            quality.unwrap_or(90),
        )
        .await
        .map_err(|e| JsValue::from_str(&format!("Render failed: {}", e)))?;
        
        Ok(js_sys::Uint8Array::from(&jpg_data[..]))
    }
    
    /// Render diagram to WebP (returns Uint8Array)
    #[wasm_bindgen(js_name = renderWebp)]
    pub async fn render_webp(
        &self,
        diagram_code: String,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f32>,
        quality: Option<f32>,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        let mut renderer = self.inner.lock().await;
        
        let webp_data = renderer.render_webp(
            &diagram_code,
            width.unwrap_or(800),
            height.unwrap_or(600),
            &background.unwrap_or_else(|| "white".to_string()),
            &theme.unwrap_or_else(|| "default".to_string()),
            scale.unwrap_or(1.0),
            quality.unwrap_or(90.0),
        )
        .await
        .map_err(|e| JsValue::from_str(&format!("Render failed: {}", e)))?;
        
        Ok(js_sys::Uint8Array::from(&webp_data[..]))
    }
    
    /// Render diagram to Data URL (for direct use in img tags)
    #[wasm_bindgen(js_name = renderDataUrl)]
    pub async fn render_data_url(
        &self,
        diagram_code: String,
        format: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
        background: Option<String>,
        theme: Option<String>,
        scale: Option<f32>,
        quality: Option<u8>,
    ) -> Result<String, JsValue> {
        let fmt = format.unwrap_or_else(|| "svg".to_string());
        
        match fmt.as_str() {
            "svg" => {
                let svg = self.render_svg(diagram_code, width, height, background, theme, scale).await?;
                let encoded = base64::encode(svg.as_bytes());
                Ok(format!("data:image/svg+xml;base64,{}", encoded))
            },
            "png" => {
                let png_data = self.render_png(diagram_code, width, height, background, theme, scale).await?;
                let bytes = png_data.to_vec();
                let encoded = base64::encode(&bytes);
                Ok(format!("data:image/png;base64,{}", encoded))
            },
            "jpg" | "jpeg" => {
                let jpg_data = self.render_jpg(diagram_code, width, height, background, theme, scale, quality).await?;
                let bytes = jpg_data.to_vec();
                let encoded = base64::encode(&bytes);
                Ok(format!("data:image/jpeg;base64,{}", encoded))
            },
            "webp" => {
                let webp_data = self.render_webp(
                    diagram_code, 
                    width, 
                    height, 
                    background, 
                    theme, 
                    scale, 
                    quality.map(|q| q as f32)
                ).await?;
                let bytes = webp_data.to_vec();
                let encoded = base64::encode(&bytes);
                Ok(format!("data:image/webp;base64,{}", encoded))
            },
            _ => Err(JsValue::from_str(&format!("Unsupported format: {}", fmt)))
        }
    }
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    set_panic_hook();
}