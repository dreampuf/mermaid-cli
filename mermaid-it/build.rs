use anyhow::Result;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = std::env::var("OUT_DIR")?;
    let mermaid_path = Path::new(&out_dir).join("mermaid.min.js");
    
    // Check if we already have the file
    if !mermaid_path.exists() {
        println!("Downloading Mermaid.js...");
        
        // Download the latest stable version of Mermaid.js from CDN
        let mermaid_url = "https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js";
        let response = reqwest::blocking::get(mermaid_url)?;
        
        if !response.status().is_success() {
            panic!("Failed to download Mermaid.js: {}", response.status());
        }
        
        let content = response.text()?;
        fs::write(&mermaid_path, content)?;
        
        println!("Mermaid.js downloaded successfully");
    }
    
    // Generate a constant with the path to the embedded file
    let const_content = format!(
        r#"pub const MERMAID_JS: &str = include_str!("{}");
"#,
        mermaid_path.display()
    );
    
    let const_path = Path::new(&out_dir).join("mermaid_const.rs");
    fs::write(const_path, const_content)?;
    
    Ok(())
}