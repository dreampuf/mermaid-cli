use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "mermaid-it")]
#[command(author = "drempuf <soddyque@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Render Mermaid diagrams to various image formats", long_about = None)]
pub struct Cli {
    /// Input file containing Mermaid diagram code (use '-' for stdin)
    #[arg(value_name = "INPUT")]
    pub input: String,
    
    /// Output file path
    #[arg(short, long, default_value = "output.svg")]
    pub output: String,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "svg")]
    pub format: OutputFormat,
    
    /// Width of the output image in pixels
    #[arg(short = 'W', long, default_value = "800")]
    pub width: u32,
    
    /// Height of the output image in pixels
    #[arg(short = 'H', long, default_value = "600")]
    pub height: u32,
    
    /// Background color (CSS color value)
    #[arg(short, long, default_value = "white")]
    pub background: String,
    
    /// Mermaid theme
    #[arg(short = 't', long, default_value = "default")]
    pub theme: String,
    
    /// Scale factor for the output
    #[arg(short, long, default_value = "1.0")]
    pub scale: f32,
    
    /// Path to custom Mermaid.js file
    #[arg(short = 'c', long)]
    pub custom_mermaid: Option<String>,
    
    /// Enable debug output
    #[arg(short = 'd', long)]
    pub debug: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Svg,
    Png,
}