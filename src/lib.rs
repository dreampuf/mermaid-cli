pub mod renderer;

#[cfg(feature = "cli")]
pub mod cli;

// Re-export main types
pub use renderer::{MermaidRenderer, RenderConfig};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Output format for rendered diagrams
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Svg,
    Png,
    Jpg,
    Jpeg,
    Webp,
    Gif,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "svg" => Some(OutputFormat::Svg),
            "png" => Some(OutputFormat::Png),
            "jpg" => Some(OutputFormat::Jpg),
            "jpeg" => Some(OutputFormat::Jpeg),
            "webp" => Some(OutputFormat::Webp),
            "gif" => Some(OutputFormat::Gif),
            _ => None,
        }
    }
}

/// Rendering options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderOptions {
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    #[serde(default = "default_background")]
    pub background: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_scale")]
    pub scale: f32,
    #[serde(default = "default_quality")]
    pub quality: u8,
}

fn default_width() -> u32 { 800 }
fn default_height() -> u32 { 600 }
fn default_background() -> String { "white".to_string() }
fn default_theme() -> String { "default".to_string() }
fn default_scale() -> f32 { 1.0 }
fn default_quality() -> u8 { 90 }

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            background: default_background(),
            theme: default_theme(),
            scale: default_scale(),
            quality: default_quality(),
        }
    }
}

/// High-level API for rendering Mermaid diagrams
pub struct MermaidIt {
    renderer: MermaidRenderer,
}

impl MermaidIt {
    /// Create a new MermaidIt instance
    pub fn new() -> Result<Self> {
        Ok(Self {
            renderer: MermaidRenderer::new()?,
        })
    }
    
    /// Set a custom Mermaid.js file content
    pub fn set_custom_mermaid(&mut self, js_content: String) {
        self.renderer.set_custom_mermaid(js_content);
    }
    
    /// Render a Mermaid diagram to the specified format
    pub async fn render(
        &mut self,
        diagram_code: &str,
        format: OutputFormat,
        options: &RenderOptions,
    ) -> Result<Vec<u8>> {
        let config = RenderConfig {
            width: options.width,
            height: options.height,
            background: options.background.clone(),
            theme: options.theme.clone(),
            scale: options.scale,
        };
        
        // First render to SVG
        let svg = self.renderer.render(diagram_code, config).await?;
        
        // Convert to requested format
        match format {
            OutputFormat::Svg => Ok(svg.into_bytes()),
            OutputFormat::Png => svg_to_png(&svg, options.width, options.height, options.scale),
            OutputFormat::Jpg | OutputFormat::Jpeg => {
                svg_to_jpg(&svg, options.width, options.height, options.scale, options.quality)
            },
            OutputFormat::Webp => {
                svg_to_webp(&svg, options.width, options.height, options.scale, options.quality as f32)
            },
            OutputFormat::Gif => {
                svg_to_gif(&svg, options.width, options.height, options.scale)
            },
        }
    }
    
    /// Render a Mermaid diagram to a string (for SVG format)
    pub async fn render_to_string(
        &mut self,
        diagram_code: &str,
        options: &RenderOptions,
    ) -> Result<String> {
        let config = RenderConfig {
            width: options.width,
            height: options.height,
            background: options.background.clone(),
            theme: options.theme.clone(),
            scale: options.scale,
        };
        
        self.renderer.render(diagram_code, config).await
    }
}

// Helper functions for image conversion
fn svg_to_png(svg_data: &str, width: u32, height: u32, scale: f32) -> Result<Vec<u8>> {
    use usvg::{Options, Tree};
    use tiny_skia::{Pixmap, Transform};
    
    let options = Options::default();
    let tree = Tree::from_str(svg_data, &options)?;
    
    let scaled_width = (width as f32 * scale) as u32;
    let scaled_height = (height as f32 * scale) as u32;
    
    let mut pixmap = Pixmap::new(scaled_width, scaled_height)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;
    
    let transform = Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    
    use image::{DynamicImage, ImageBuffer, Rgba};
    let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
        scaled_width,
        scaled_height,
        pixmap.data().to_vec(),
    ).ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut png_data = Vec::new();
    dynamic_img.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageFormat::Png)?;
    
    Ok(png_data)
}

fn svg_to_jpg(svg_data: &str, width: u32, height: u32, scale: f32, quality: u8) -> Result<Vec<u8>> {
    use usvg::{Options, Tree};
    use tiny_skia::{Pixmap, Transform};
    
    let options = Options::default();
    let tree = Tree::from_str(svg_data, &options)?;
    
    let scaled_width = (width as f32 * scale) as u32;
    let scaled_height = (height as f32 * scale) as u32;
    
    let mut pixmap = Pixmap::new(scaled_width, scaled_height)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;
    
    let transform = Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    
    use image::{DynamicImage, ImageBuffer, Rgba};
    let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
        scaled_width,
        scaled_height,
        pixmap.data().to_vec(),
    ).ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut jpg_data = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpg_data, quality);
    encoder.encode_image(&dynamic_img)?;
    
    Ok(jpg_data)
}

fn svg_to_webp(svg_data: &str, width: u32, height: u32, scale: f32, quality: f32) -> Result<Vec<u8>> {
    use usvg::{Options, Tree};
    use tiny_skia::{Pixmap, Transform};
    
    let options = Options::default();
    let tree = Tree::from_str(svg_data, &options)?;
    
    let scaled_width = (width as f32 * scale) as u32;
    let scaled_height = (height as f32 * scale) as u32;
    
    let mut pixmap = Pixmap::new(scaled_width, scaled_height)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;
    
    let transform = Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    
    let encoder = webp::Encoder::from_rgba(pixmap.data(), scaled_width, scaled_height);
    let webp_data = encoder.encode(quality);
    
    Ok(webp_data.to_vec())
}

fn svg_to_gif(svg_data: &str, width: u32, height: u32, scale: f32) -> Result<Vec<u8>> {
    use usvg::{Options, Tree};
    use tiny_skia::{Pixmap, Transform};
    
    let options = Options::default();
    let tree = Tree::from_str(svg_data, &options)?;
    
    let scaled_width = (width as f32 * scale) as u32;
    let scaled_height = (height as f32 * scale) as u32;
    
    let mut pixmap = Pixmap::new(scaled_width, scaled_height)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;
    
    let transform = Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    
    use image::{DynamicImage, ImageBuffer, Rgba};
    let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
        scaled_width,
        scaled_height,
        pixmap.data().to_vec(),
    ).ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut gif_data = Vec::new();
    dynamic_img.write_to(&mut std::io::Cursor::new(&mut gif_data), image::ImageFormat::Gif)?;
    
    Ok(gif_data)
}

// UniFFI bindings for multiple languages
#[cfg(feature = "uniffi-bindings")]
pub mod uniffi_bindings;

// C FFI bindings for Go and other languages
#[cfg(feature = "uniffi-bindings")]
pub mod c_bindings;

// WASM bindings (works for both browser and Node.js)
#[cfg(feature = "wasm")]
pub mod wasm_bindings;