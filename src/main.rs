mod renderer;
mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, OutputFormat};
use renderer::MermaidRenderer;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Read the input Mermaid diagram
    let diagram_code = if cli.input == "-" {
        // Read from stdin
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        // Read from file
        fs::read_to_string(&cli.input)?
    };
    
    // Create the renderer
    let mut renderer = MermaidRenderer::new()?;
    
    // Load custom Mermaid.js if provided
    if let Some(custom_js_path) = &cli.custom_mermaid {
        let custom_js = fs::read_to_string(custom_js_path)?;
        renderer.set_custom_mermaid(custom_js);
    }
    
    // Configure rendering options
    let config = renderer::RenderConfig {
        width: cli.width,
        height: cli.height,
        background: cli.background.clone(),
        theme: cli.theme.clone(),
        scale: cli.scale,
    };
    
    // Render the diagram
    let svg_output = renderer.render(&diagram_code, config).await?;
    
    // Convert and save based on output format
    match cli.format {
        OutputFormat::Svg => {
            fs::write(&cli.output, svg_output)?;
        }
        OutputFormat::Png => {
            let image_data = convert_svg_to_raster(&svg_output, cli.width, cli.height, cli.scale, ImageFormat::Png)?;
            fs::write(&cli.output, image_data)?;
        }
        OutputFormat::Jpg => {
            let image_data = convert_svg_to_raster(&svg_output, cli.width, cli.height, cli.scale, ImageFormat::Jpeg)?;
            fs::write(&cli.output, image_data)?;
        }
        OutputFormat::Webp => {
            let image_data = convert_svg_to_raster(&svg_output, cli.width, cli.height, cli.scale, ImageFormat::WebP)?;
            fs::write(&cli.output, image_data)?;
        }
        OutputFormat::Gif => {
            let image_data = convert_svg_to_raster(&svg_output, cli.width, cli.height, cli.scale, ImageFormat::Gif)?;
            fs::write(&cli.output, image_data)?;
        }
    }
    
    println!("✓ Diagram rendered successfully to: {}", cli.output);
    
    Ok(())
}

enum ImageFormat {
    Png,
    Jpeg,
    WebP,
    Gif,
}

fn convert_svg_to_raster(svg_data: &str, width: u32, height: u32, scale: f32, format: ImageFormat) -> Result<Vec<u8>> {
    use usvg::{Options, Tree};
    use tiny_skia::{Pixmap, Transform};
    use resvg::render;
    use image::{DynamicImage, ImageBuffer, Rgba};
    
    let opt = Options::default();
    let tree = Tree::from_str(svg_data, &opt)?;
    
    let scaled_width = (width as f32 * scale) as u32;
    let scaled_height = (height as f32 * scale) as u32;
    
    let mut pixmap = Pixmap::new(scaled_width, scaled_height)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;
    
    // Fill with white background for formats that don't support transparency
    pixmap.fill(tiny_skia::Color::WHITE);
    
    let transform = Transform::from_scale(scale, scale);
    render(&tree, transform, &mut pixmap.as_mut());
    
    // Convert pixmap to image buffer
    let rgba_data = pixmap.data();
    let img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
        scaled_width,
        scaled_height,
        rgba_data.to_vec()
    ).ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;
    
    let dynamic_image = DynamicImage::ImageRgba8(img_buffer);
    
    // Encode to the requested format
    let mut output = Vec::new();
    use std::io::Cursor;
    let mut cursor = Cursor::new(&mut output);
    
    match format {
        ImageFormat::Png => {
            dynamic_image.write_to(&mut cursor, image::ImageFormat::Png)?;
        }
        ImageFormat::Jpeg => {
            // Convert to RGB for JPEG (no alpha channel)
            let rgb_image = dynamic_image.to_rgb8();
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, 90)
                .encode(&rgb_image, rgb_image.width(), rgb_image.height(), image::ExtendedColorType::Rgb8)?;
        }
        ImageFormat::WebP => {
            // Use webp crate for WebP encoding
            let rgba_image = dynamic_image.to_rgba8();
            let encoder = webp::Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height());
            let webp_data = encoder.encode(90.0);
            output = webp_data.to_vec();
        }
        ImageFormat::Gif => {
            dynamic_image.write_to(&mut cursor, image::ImageFormat::Gif)?;
        }
    }
    
    Ok(output)
}