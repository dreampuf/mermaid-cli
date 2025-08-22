use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Download and embed Mermaid.js
    download_mermaid()?;
    
    // Generate UniFFI bindings if feature is enabled
    #[cfg(feature = "uniffi-bindings")]
    {
        uniffi::generate_scaffolding("src/mermaid_it.udl")?;
    }
    
    Ok(())
}

fn download_mermaid() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let mermaid_path = out_dir.join("mermaid.min.js");
    let const_path = out_dir.join("mermaid_const.rs");
    
    // Check if we already have the file
    if mermaid_path.exists() && const_path.exists() {
        println!("cargo:rerun-if-changed=build.rs");
        return Ok(());
    }
    
    println!("cargo:warning=Downloading Mermaid.js...");
    
    // Download Mermaid.js from CDN
    let url = "https://cdn.jsdelivr.net/npm/mermaid@10.6.1/dist/mermaid.min.js";
    let response = reqwest::blocking::get(url)?;
    let mermaid_content = response.text()?;
    
    // Save the file
    fs::write(&mermaid_path, &mermaid_content)?;
    
    // Generate Rust constant
    let const_content = format!(
        r#"pub const MERMAID_JS: &str = r###"{}"###;"#,
        mermaid_content
    );
    fs::write(&const_path, const_content)?;
    
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}