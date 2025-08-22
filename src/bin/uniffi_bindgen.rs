use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <language> [output_dir]", args[0]);
        eprintln!("Supported languages: kotlin, swift, python, ruby");
        std::process::exit(1);
    }
    
    let language = &args[1];
    let output_dir = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from(format!("bindings/{}", language))
    };
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    // Path to the UDL file
    let udl_file = PathBuf::from("src/mermaid_it.udl");
    
    println!("Generating {} bindings in {:?}...", language, output_dir);
    
    // Generate bindings using uniffi-bindgen
    match language.as_str() {
        "kotlin" | "swift" | "python" | "ruby" => {
            uniffi::generate_bindings(
                &udl_file,
                None,
                vec![language.into()],
                Some(&output_dir),
                None,
                false,
            ).expect("Failed to generate bindings");
            
            println!("Successfully generated {} bindings!", language);
        }
        _ => {
            eprintln!("Unsupported language: {}", language);
            eprintln!("Supported languages: kotlin, swift, python, ruby");
            std::process::exit(1);
        }
    }
}