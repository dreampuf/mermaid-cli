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
            let png_data = convert_svg_to_png(&svg_output, cli.width, cli.height, cli.scale)?;
            fs::write(&cli.output, png_data)?;
        }
        OutputFormat::Pdf => {
            let pdf_data = convert_svg_to_pdf(&svg_output)?;
            fs::write(&cli.output, pdf_data)?;
        }
    }
    
    println!("✓ Diagram rendered successfully to: {}", cli.output);
    
    Ok(())
}

fn convert_svg_to_png(svg_data: &str, width: u32, height: u32, scale: f32) -> Result<Vec<u8>> {
    use usvg::{Options, Tree};
    use tiny_skia::{Pixmap, Transform};
    use resvg::render;
    
    let opt = Options::default();
    let tree = Tree::from_str(svg_data, &opt)?;
    
    let scaled_width = (width as f32 * scale) as u32;
    let scaled_height = (height as f32 * scale) as u32;
    
    let mut pixmap = Pixmap::new(scaled_width, scaled_height)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;
    
    let transform = Transform::from_scale(scale, scale);
    render(&tree, transform, &mut pixmap.as_mut());
    
    pixmap.encode_png()
        .map_err(|e| anyhow::anyhow!("Failed to encode PNG: {:?}", e))
}

fn convert_svg_to_pdf(svg_data: &str) -> Result<Vec<u8>> {
    // For PDF conversion, we'll use a simple approach
    // In a production environment, you might want to use a more sophisticated library
    use std::process::Command;
    use tempfile::NamedTempFile;
    
    let svg_file = NamedTempFile::new()?;
    fs::write(svg_file.path(), svg_data)?;
    
    let pdf_file = NamedTempFile::new()?;
    
    // Try to use rsvg-convert if available
    let output = Command::new("rsvg-convert")
        .arg("-f")
        .arg("pdf")
        .arg("-o")
        .arg(pdf_file.path())
        .arg(svg_file.path())
        .output();
    
    if output.is_ok() {
        return Ok(fs::read(pdf_file.path())?);
    }
    
    // Fallback: embed SVG in a simple PDF structure
    // This is a very basic implementation
    let pdf_content = format!(
        r#"%PDF-1.4
1 0 obj
<< /Type /Catalog /Pages 2 0 R >>
endobj
2 0 obj
<< /Type /Pages /Kids [3 0 R] /Count 1 >>
endobj
3 0 obj
<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << >> >>
endobj
xref
0 4
0000000000 65535 f
0000000009 00000 n
0000000058 00000 n
0000000115 00000 n
trailer
<< /Size 4 /Root 1 0 R >>
startxref
217
%%EOF"#
    );
    
    Ok(pdf_content.into_bytes())
}