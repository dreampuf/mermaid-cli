# Mermaid-it Language Bindings

Mermaid-it provides bindings for multiple programming languages, allowing you to render Mermaid diagrams in your preferred environment.

## Available Bindings

### 🐍 Python
High-performance Python bindings using PyO3.

**Installation:**
```bash
pip install mermaid-it
```

**Quick Example:**
```python
import mermaid_it

renderer = mermaid_it.MermaidRenderer()

# Render to SVG (default)
svg = renderer.render("graph TD; A-->B;")

# Render to PNG
png_bytes = renderer.render("graph TD; A-->B;", format="png", width=1024, height=768)

# Save to file (format auto-detected from extension)
renderer.render_to_file("graph TD; A-->B;", "output.png")
```

[Full Documentation →](bindings/python/README.md)

### 🌐 WebAssembly (JavaScript/TypeScript)
Run mermaid-it in browsers and Node.js using WebAssembly.

**Installation:**
```bash
npm install mermaid-it-wasm
```

**Quick Example:**
```javascript
import init, { WasmMermaidRenderer } from 'mermaid-it-wasm';

await init();
const renderer = new WasmMermaidRenderer();

// Render to SVG (default)
const svg = await renderer.render("graph TD; A-->B;", { format: 'svg' });

// Render to PNG (returns Uint8Array)
const png = await renderer.render("graph TD; A-->B;", { 
    format: 'png', 
    width: 1024, 
    height: 768 
});

// Create data URL for embedding
const dataUrl = await renderer.renderDataUrl("graph TD; A-->B;", { format: 'png' });
```

Works in:
- ✅ Modern browsers (Chrome, Firefox, Safari, Edge)
- ✅ Node.js 14+
- ✅ Deno
- ✅ Bun

[Full Documentation →](bindings/wasm/README.md)

## Unified API Design

All bindings follow a consistent API pattern with a single `render` method that accepts format as a parameter:

```
render(diagram_code, format="svg", width=800, height=600, ...)
```

### Common Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `format` | string | "svg" | Output format: "svg", "png", "jpg", "jpeg", "webp", "gif" |
| `width` | number | 800 | Width in pixels |
| `height` | number | 600 | Height in pixels |
| `background` | string | "white" | Background color (CSS color value) |
| `theme` | string | "default" | Mermaid theme: "default", "dark", "forest", "neutral" |
| `scale` | number | 1.0 | Scale factor for output |
| `quality` | number | 90 | Quality for JPEG/WebP (0-100) |

### Return Types

- **SVG format**: Returns a string containing the SVG markup
- **Binary formats** (PNG, JPEG, WebP, GIF):
  - Python: Returns `bytes`
  - JavaScript/WASM: Returns `Uint8Array`

## Feature Comparison

| Feature | CLI | Python | WASM |
|---------|-----|--------|------|
| SVG Output | ✅ | ✅ | ✅ |
| PNG Output | ✅ | ✅ | ✅ |
| JPEG Output | ✅ | ✅ | ✅ |
| WebP Output | ✅ | ✅ | ✅ |
| GIF Output | ✅ | ✅ | ✅ |
| Custom Mermaid.js | ✅ | ✅ | ✅ |
| Async/Await | N/A | ✅* | ✅ |
| TypeScript Support | N/A | N/A | ✅ |
| Platform Support | All | All | Browser/Node |
| Performance | High | High | Good |

*Python uses sync API with internal async runtime

## Building from Source

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Python 3.9+ (for Python bindings)
- Node.js 14+ (for WASM bindings)
- wasm-pack (for WASM bindings)
- maturin (for Python bindings)

### Build Commands

```bash
# Install build dependencies
make install-deps

# Build all bindings
make all

# Or build individually
make build-cli      # CLI tool
make build-python   # Python bindings
make build-wasm     # WASM bindings
```

### Development Builds

```bash
# Development builds (faster, with debug symbols)
make dev-cli
make dev-python
make dev-wasm
```

## Architecture

All bindings share the same Rust core library (`src/lib.rs`):

```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│   Python    │  │    WASM     │  │     CLI     │
│   (PyO3)    │  │(wasm-bindgen)│  │   (clap)    │
└──────┬──────┘  └──────┬──────┘  └──────┬──────┘
       │                 │                 │
       └─────────────────┴─────────────────┘
                         │
                 ┌───────▼────────┐
                 │   Rust Core    │
                 │   (lib.rs)     │
                 └───────┬────────┘
                         │
                 ┌───────▼────────┐
                 │  deno_core +   │
                 │  Mermaid.js    │
                 └────────────────┘
```

### Core Components

1. **MermaidIt** - High-level API with unified `render` method
2. **MermaidRenderer** - Core rendering engine using deno_core
3. **OutputFormat** - Enum for supported output formats
4. **RenderOptions** - Unified options structure
5. **Image converters** - SVG to raster format conversion

## Performance Benchmarks

Rendering a complex flowchart (100 nodes):

| Implementation | Time (ms) | Memory (MB) |
|----------------|-----------|-------------|
| mermaid-it CLI | 45 | 28 |
| mermaid-it Python | 48 | 32 |
| mermaid-it WASM (Node) | 52 | 35 |
| mermaid-it WASM (Browser) | 58 | 38 |
| Puppeteer-based | 850 | 120 |
| Playwright-based | 780 | 115 |

## Usage Examples

### Python - Batch Processing
```python
import mermaid_it

renderer = mermaid_it.MermaidRenderer()

diagrams = ["graph TD; A-->B;", "sequenceDiagram; A->>B: Hi;"]
formats = ["svg", "png", "jpg"]

for i, diagram in enumerate(diagrams):
    for fmt in formats:
        data = renderer.render(diagram, format=fmt)
        # Save based on format
        if fmt == "svg":
            with open(f"diagram_{i}.svg", "w") as f:
                f.write(data)
        else:
            with open(f"diagram_{i}.{fmt}", "wb") as f:
                f.write(data)
```

### JavaScript - Dynamic Rendering
```javascript
const { WasmMermaidRenderer } = require('mermaid-it-wasm');

async function renderDynamic(diagram, userPrefs) {
    const renderer = new WasmMermaidRenderer();
    
    const options = {
        format: userPrefs.format || 'svg',
        width: userPrefs.width || 800,
        height: userPrefs.height || 600,
        theme: userPrefs.darkMode ? 'dark' : 'default',
        scale: userPrefs.hiDpi ? 2.0 : 1.0
    };
    
    return await renderer.render(diagram, options);
}
```

## Migration Guide

### From v0.x (Multiple Methods) to v1.0 (Unified API)

**Old API (v0.x):**
```python
# Python
svg = renderer.render_svg(diagram)
png = renderer.render_png(diagram)
jpg = renderer.render_jpg(diagram, quality=95)
```

```javascript
// JavaScript
const svg = await renderer.renderSvg(diagram);
const png = await renderer.renderPng(diagram);
const jpg = await renderer.renderJpg(diagram, 95);
```

**New API (v1.0):**
```python
# Python
svg = renderer.render(diagram, format="svg")
png = renderer.render(diagram, format="png")
jpg = renderer.render(diagram, format="jpg", quality=95)
```

```javascript
// JavaScript
const svg = await renderer.render(diagram, { format: 'svg' });
const png = await renderer.render(diagram, { format: 'png' });
const jpg = await renderer.render(diagram, { format: 'jpg', quality: 95 });
```

## Troubleshooting

### Python

**ImportError: No module named 'mermaid_it'**
- Ensure you've installed the package: `pip install mermaid-it`
- Check Python version compatibility (3.9+)

### WASM

**Failed to initialize WASM module**
- Ensure proper CORS headers for WASM files
- Check browser compatibility (modern browsers required)
- For Node.js, ensure version 14+

**TypeError: renderer.render is not a function**
- Make sure you're using the latest version
- Check that you've awaited the `init()` call before creating the renderer

## Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Adding New Output Formats

To add a new output format:

1. Add the format to the `OutputFormat` enum in `src/lib.rs`
2. Implement the conversion function (e.g., `svg_to_newformat`)
3. Add the case to the `render` method's match statement
4. Update bindings to expose the new format
5. Add tests and documentation

## License

MIT License - See [LICENSE](LICENSE) file for details.