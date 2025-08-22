pub mod renderer;

#[cfg(feature = "cli")]
pub mod cli;

// Re-export main types
pub use renderer::{MermaidRenderer, RenderConfig};

use anyhow::Result;

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
    
    /// Render a Mermaid diagram to SVG
    pub async fn render_svg(
        &mut self,
        diagram_code: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
    ) -> Result<String> {
        let config = RenderConfig {
            width,
            height,
            background: background.to_string(),
            theme: theme.to_string(),
            scale,
        };
        
        self.renderer.render(diagram_code, config).await
    }
    
    /// Render a Mermaid diagram to PNG
    pub async fn render_png(
        &mut self,
        diagram_code: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
    ) -> Result<Vec<u8>> {
        let svg = self.render_svg(diagram_code, width, height, background, theme, scale).await?;
        svg_to_png(&svg, width, height, scale)
    }
    
    /// Render a Mermaid diagram to JPEG
    pub async fn render_jpg(
        &mut self,
        diagram_code: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
        quality: u8,
    ) -> Result<Vec<u8>> {
        let svg = self.render_svg(diagram_code, width, height, background, theme, scale).await?;
        svg_to_jpg(&svg, width, height, scale, quality)
    }
    
    /// Render a Mermaid diagram to WebP
    pub async fn render_webp(
        &mut self,
        diagram_code: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
        quality: f32,
    ) -> Result<Vec<u8>> {
        let svg = self.render_svg(diagram_code, width, height, background, theme, scale).await?;
        svg_to_webp(&svg, width, height, scale, quality)
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

// Python bindings
#[cfg(feature = "python")]
pub mod python_bindings;

// WASM bindings
#[cfg(feature = "wasm")]
pub mod wasm_bindings;

// Node.js bindings
#[cfg(feature = "nodejs")]
pub mod nodejs_bindings;