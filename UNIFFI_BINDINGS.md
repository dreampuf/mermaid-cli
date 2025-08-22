# UniFFI Language Bindings for Mermaid-it

Mermaid-it now uses **UniFFI** as the primary binding solution, providing native bindings for multiple languages from a single interface definition.

## Supported Languages

### Tier 1 - Native UniFFI Support
- **Python** - Full support with pythonic API
- **Ruby** - Full support with Ruby conventions
- **Kotlin** - Full support for Android/JVM
- **Swift** - Full support for iOS/macOS

### Tier 2 - Via C FFI
- **Go** - Using CGO with C bindings
- **Java** - Using Kotlin bindings via JNI

### Tier 3 - WebAssembly
- **JavaScript/TypeScript** - Browser and Node.js via WASM

## Quick Start

### 1. Build the Library

```bash
# Build with UniFFI support
cargo build --release --features uniffi-bindings

# Generate all bindings
make generate-bindings

# Or generate specific language
make generate-python
make generate-ruby
make generate-kotlin
make generate-swift
```

### 2. Language-Specific Usage

## Python

```python
import mermaid_it

# Create renderer
renderer = mermaid_it.MermaidRenderer()

# Render with options
options = mermaid_it.RenderOptions(
    format="png",
    width=1024,
    height=768,
    theme="dark",
    scale=2.0
)

# Get bytes for binary formats
png_data = renderer.render("graph TD; A-->B;", options)

# Get string for SVG
svg_options = mermaid_it.RenderOptions(format="svg")
svg = renderer.render_to_string("graph TD; A-->B;", svg_options)

# Save to file
renderer.render_to_file("graph TD; A-->B;", "output.png", options)
```

## Ruby

```ruby
require 'mermaid_it'

# Create renderer
renderer = MermaidIt::MermaidRenderer.new

# Create options
options = MermaidIt::RenderOptions.new
options.format = "png"
options.width = 1024
options.height = 768
options.theme = "dark"

# Render diagram
png_data = renderer.render("graph TD; A-->B;", options)

# Get SVG as string
svg_options = MermaidIt::RenderOptions.new
svg_options.format = "svg"
svg = renderer.render_to_string("graph TD; A-->B;", svg_options)

# Save to file
renderer.render_to_file("graph TD; A-->B;", "output.png", options)
```

## Kotlin

```kotlin
import mermaid_it.*

fun main() {
    // Create renderer
    val renderer = MermaidRenderer()
    
    // Create options
    val options = RenderOptions(
        format = "png",
        width = 1024u,
        height = 768u,
        theme = "dark",
        scale = 2.0f
    )
    
    // Render diagram
    val pngData: ByteArray = renderer.render("graph TD; A-->B;", options)
    
    // Get SVG as string
    val svgOptions = RenderOptions(format = "svg")
    val svg: String = renderer.renderToString("graph TD; A-->B;", svgOptions)
    
    // Save to file
    renderer.renderToFile("graph TD; A-->B;", "output.png", options)
}
```

## Swift

```swift
import MermaidIt

// Create renderer
let renderer = MermaidRenderer()

// Create options
let options = RenderOptions(
    format: "png",
    width: 1024,
    height: 768,
    background: "white",
    theme: "dark",
    scale: 2.0,
    quality: 90
)

do {
    // Render diagram
    let pngData = try renderer.render(diagram: "graph TD; A-->B;", options: options)
    
    // Get SVG as string
    let svgOptions = RenderOptions(format: "svg")
    let svg = try renderer.renderToString(diagram: "graph TD; A-->B;", options: svgOptions)
    
    // Save to file
    try renderer.renderToFile(
        diagram: "graph TD; A-->B;",
        outputPath: "output.png",
        options: options
    )
} catch {
    print("Error: \(error)")
}
```

## Go

```go
package main

import (
    "fmt"
    "io/ioutil"
    "github.com/dreampuf/mermaid-it/bindings/go"
)

func main() {
    // Create renderer
    renderer, err := mermaid.NewRenderer()
    if err != nil {
        panic(err)
    }
    defer renderer.Close()
    
    // Create options
    options := mermaid.RenderOptions{
        Format:     mermaid.FormatPNG,
        Width:      1024,
        Height:     768,
        Background: "white",
        Theme:      mermaid.ThemeDark,
        Scale:      2.0,
        Quality:    90,
    }
    
    // Render diagram
    pngData, err := renderer.Render("graph TD; A-->B;", options)
    if err != nil {
        panic(err)
    }
    
    // Save to file
    err = ioutil.WriteFile("output.png", pngData, 0644)
    if err != nil {
        panic(err)
    }
    
    // Get SVG as string
    svgOptions := mermaid.DefaultOptions()
    svgOptions.Format = mermaid.FormatSVG
    svg, err := renderer.RenderToString("graph TD; A-->B;", svgOptions)
    if err != nil {
        panic(err)
    }
    fmt.Println(svg)
}
```

## Java (via Kotlin/JNI)

```java
import mermaid_it.*;

public class MermaidExample {
    public static void main(String[] args) {
        // Create renderer
        MermaidRenderer renderer = new MermaidRenderer();
        
        // Create options
        RenderOptions options = new RenderOptions(
            "png",    // format
            1024,     // width
            768,      // height
            "white",  // background
            "dark",   // theme
            2.0f,     // scale
            (byte)90  // quality
        );
        
        try {
            // Render diagram
            byte[] pngData = renderer.render("graph TD; A-->B;", options);
            
            // Get SVG as string
            RenderOptions svgOptions = new RenderOptions();
            svgOptions.setFormat("svg");
            String svg = renderer.renderToString("graph TD; A-->B;", svgOptions);
            
            // Save to file
            renderer.renderToFile("graph TD; A-->B;", "output.png", options);
        } catch (MermaidException e) {
            e.printStackTrace();
        }
    }
}
```

## API Reference

### Common Types

#### RenderOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| format | string | "svg" | Output format: svg, png, jpg, jpeg, webp, gif |
| width | u32 | 800 | Width in pixels |
| height | u32 | 600 | Height in pixels |
| background | string | "white" | Background color (CSS value) |
| theme | string | "default" | Mermaid theme: default, dark, forest, neutral |
| scale | f32 | 1.0 | Scale factor |
| quality | u8 | 90 | JPEG/WebP quality (0-100) |

### MermaidRenderer Methods

#### render(diagram: string, options: RenderOptions) -> bytes
Renders a diagram and returns the raw bytes.

#### render_to_string(diagram: string, options: RenderOptions) -> string
Renders a diagram and returns it as a string (useful for SVG).

#### render_to_file(diagram: string, path: string, options: RenderOptions)
Renders a diagram directly to a file.

#### set_custom_mermaid(js_content: string)
Sets custom Mermaid.js content for rendering.

## Building from Source

### Prerequisites

- Rust 1.70+
- For Go: GCC/Clang for CGO
- For Swift: Xcode (macOS)
- For Kotlin: JDK 8+

### Build Steps

```bash
# Clone the repository
git clone https://github.com/dreampuf/mermaid-it
cd mermaid-it

# Build the Rust library
cargo build --release --features uniffi-bindings

# Generate bindings for all languages
./generate_bindings.sh

# Or generate for specific language
cargo run --bin uniffi-bindgen --features uniffi-bindings -- python
cargo run --bin uniffi-bindgen --features uniffi-bindings -- ruby
cargo run --bin uniffi-bindgen --features uniffi-bindings -- kotlin
cargo run --bin uniffi-bindgen --features uniffi-bindings -- swift
```

## Packaging and Distribution

### Python
```bash
cd bindings/python
pip install .
# Or publish to PyPI
python setup.py sdist bdist_wheel
twine upload dist/*
```

### Ruby
```bash
cd bindings/ruby
gem build mermaid_it.gemspec
gem push mermaid_it-*.gem
```

### Kotlin/Java
```bash
cd bindings/kotlin
./gradlew build
# Publish to Maven Central
./gradlew publish
```

### Swift
```bash
cd bindings/swift
# Create Swift Package
swift package init --type library
# Add to Package.swift
```

### Go
```bash
# The Go package can be imported directly
go get github.com/dreampuf/mermaid-it/bindings/go
```

## Performance Comparison

| Language | Overhead | Use Case |
|----------|----------|----------|
| Rust (native) | 0% | Maximum performance |
| Go (CGO) | ~5% | High performance with Go ecosystem |
| Kotlin/Java | ~8% | Android/JVM applications |
| Swift | ~5% | iOS/macOS applications |
| Python | ~10% | Data science, scripting |
| Ruby | ~12% | Web applications, scripting |
| WASM | ~15% | Browser, universal JavaScript |

## Troubleshooting

### UniFFI Generation Errors

**Error: "Failed to parse UDL file"**
- Ensure the UDL file syntax is correct
- Check that all types are properly defined

**Error: "Cannot find uniffi-bindgen"**
- Build with `--features uniffi-bindings` flag
- Run `cargo build --release --features uniffi-bindings` first

### Language-Specific Issues

**Python: "No module named 'mermaid_it'"**
- Ensure the Python package is installed
- Check that the `.so`/`.dylib`/`.dll` file is in the package

**Go: "undefined reference to mermaid_renderer_new"**
- Ensure the Rust library is built: `cargo build --release --features uniffi-bindings`
- Check CGO_LDFLAGS points to the correct library path

**Kotlin/Java: "UnsatisfiedLinkError"**
- Ensure the native library is in the library path
- Set `-Djava.library.path=/path/to/lib`

**Swift: "Module 'MermaidIt' not found"**
- Ensure the Swift package is properly configured
- Check that the `.xcframework` is included

## Contributing

To add support for a new language:

1. Check if UniFFI supports it natively
2. If not, consider using the C FFI interface
3. Create bindings in `bindings/<language>/`
4. Add build configuration to `Makefile`
5. Add examples and documentation
6. Submit a pull request

## License

MIT License - See LICENSE file for details.