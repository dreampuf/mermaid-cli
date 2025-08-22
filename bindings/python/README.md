# Mermaid-it Python Bindings

Python bindings for mermaid-it, a fast Mermaid diagram renderer using Rust.

## Installation

```bash
pip install mermaid-it
```

## Quick Start

```python
import mermaid_it

# Create a renderer
renderer = mermaid_it.MermaidRenderer()

# Define your diagram
diagram = """
graph TD
    A[Start] --> B{Is it working?}
    B -->|Yes| C[Great!]
    B -->|No| D[Debug]
    D --> B
    C --> E[End]
"""

# Render to SVG (default)
svg = renderer.render(diagram)
print(svg)  # SVG string

# Render to PNG
png_bytes = renderer.render(
    diagram,
    format="png",
    width=1024,
    height=768,
    background="#f0f0f0",
    theme="dark",
    scale=2.0
)
# png_bytes is a bytes object

# Save directly to file
renderer.render_to_file(diagram, "output.png")  # Format inferred from extension
renderer.render_to_file(diagram, "output.svg", format="svg")  # Or specify explicitly
```

## API Reference

### MermaidRenderer

The main class for rendering Mermaid diagrams.

#### Constructor

```python
renderer = mermaid_it.MermaidRenderer()
```

#### Methods

##### render(diagram_code, format="svg", width=800, height=600, background="white", theme="default", scale=1.0, quality=90)

Render a diagram to the specified format.

**Parameters:**
- `diagram_code` (str): The Mermaid diagram code
- `format` (str): Output format - "svg", "png", "jpg", "jpeg", "webp", "gif" (default: "svg")
- `width` (int): Width in pixels (default: 800)
- `height` (int): Height in pixels (default: 600)
- `background` (str): Background color (default: "white")
- `theme` (str): Mermaid theme (default: "default")
- `scale` (float): Scale factor (default: 1.0)
- `quality` (int): Quality for JPEG/WebP formats, 0-100 (default: 90)

**Returns:** 
- For SVG: `str` - The SVG content
- For other formats: `bytes` - The image data

##### render_to_file(diagram_code, output_path, format=None, width=800, height=600, background="white", theme="default", scale=1.0, quality=90)

Render a diagram directly to a file.

**Parameters:**
- `output_path` (str): Path to save the file
- `format` (str, optional): Output format. If not specified, inferred from file extension
- Other parameters same as `render()`

##### set_custom_mermaid(js_content)

Set custom Mermaid.js content for rendering.

**Parameters:**
- `js_content` (str): The Mermaid.js JavaScript content

## Examples

### Different Output Formats

```python
import mermaid_it

renderer = mermaid_it.MermaidRenderer()

diagram = """
sequenceDiagram
    participant Alice
    participant Bob
    Alice->>Bob: Hello Bob!
    Bob-->>Alice: Hi Alice!
"""

# SVG (returns string)
svg = renderer.render(diagram, format="svg")

# PNG (returns bytes)
png = renderer.render(diagram, format="png", width=1200, height=800)

# JPEG with quality setting
jpg = renderer.render(diagram, format="jpg", quality=95)

# WebP with transparency
webp = renderer.render(diagram, format="webp", background="transparent")
```

### Batch Processing

```python
import mermaid_it
import os

renderer = mermaid_it.MermaidRenderer()

diagrams = {
    "flowchart": "graph TD\n    A-->B\n    B-->C",
    "sequence": "sequenceDiagram\n    Alice->>Bob: Hi",
    "gantt": "gantt\n    title Project\n    Task1 :a1, 2024-01-01, 7d"
}

for name, code in diagrams.items():
    # Auto-detect format from extension
    renderer.render_to_file(code, f"{name}.svg")
    renderer.render_to_file(code, f"{name}.png", width=1200)
```

### Custom Themes

```python
import mermaid_it

renderer = mermaid_it.MermaidRenderer()

diagram = "graph TD\n    A-->B"

# Available themes: default, dark, forest, neutral
for theme in ["default", "dark", "forest", "neutral"]:
    svg = renderer.render(diagram, theme=theme)
    with open(f"diagram_{theme}.svg", "w") as f:
        f.write(svg)
```

## Supported Diagram Types

- Flowcharts
- Sequence diagrams
- Gantt charts
- Class diagrams
- State diagrams
- Entity Relationship diagrams
- User Journey diagrams
- Git graphs
- Pie charts
- Requirement diagrams
- And more!

## Performance

The Python bindings use native Rust code for maximum performance:
- 10-50x faster than browser-based solutions
- Minimal memory footprint
- No browser or Node.js overhead
- Thread-safe for concurrent rendering

## License

MIT License