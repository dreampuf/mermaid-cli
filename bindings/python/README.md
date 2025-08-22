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

# Render to SVG
svg = renderer.render_svg(diagram)
with open("output.svg", "w") as f:
    f.write(svg)

# Render to PNG
png_bytes = renderer.render_png(
    diagram,
    width=1024,
    height=768,
    background="#f0f0f0",
    theme="dark",
    scale=2.0
)
with open("output.png", "wb") as f:
    f.write(png_bytes)
```

## API Reference

### MermaidRenderer

The main class for rendering Mermaid diagrams.

#### Constructor

```python
renderer = mermaid_it.MermaidRenderer()
```

#### Methods

##### render_svg(diagram_code, width=800, height=600, background="white", theme="default", scale=1.0)

Render a diagram to SVG format.

**Parameters:**
- `diagram_code` (str): The Mermaid diagram code
- `width` (int): Width in pixels (default: 800)
- `height` (int): Height in pixels (default: 600)
- `background` (str): Background color (default: "white")
- `theme` (str): Mermaid theme (default: "default")
- `scale` (float): Scale factor (default: 1.0)

**Returns:** str - The SVG content

##### render_png(diagram_code, width=800, height=600, background="white", theme="default", scale=1.0)

Render a diagram to PNG format.

**Returns:** bytes - The PNG image data

##### render_jpg(diagram_code, width=800, height=600, background="white", theme="default", scale=1.0, quality=90)

Render a diagram to JPEG format.

**Parameters:**
- `quality` (int): JPEG quality 0-100 (default: 90)

**Returns:** bytes - The JPEG image data

##### render_webp(diagram_code, width=800, height=600, background="white", theme="default", scale=1.0, quality=90.0)

Render a diagram to WebP format.

**Parameters:**
- `quality` (float): WebP quality 0-100 (default: 90.0)

**Returns:** bytes - The WebP image data

##### render_to_file(diagram_code, output_path, format="svg", width=800, height=600, background="white", theme="default", scale=1.0, quality=90)

Render a diagram directly to a file.

**Parameters:**
- `output_path` (str): Path to save the file
- `format` (str): Output format ("svg", "png", "jpg", "webp")
- Other parameters same as above

##### set_custom_mermaid(js_content)

Set custom Mermaid.js content for rendering.

**Parameters:**
- `js_content` (str): The Mermaid.js JavaScript content

## Examples

### Sequence Diagram

```python
import mermaid_it

renderer = mermaid_it.MermaidRenderer()

sequence = """
sequenceDiagram
    participant Alice
    participant Bob
    Alice->>Bob: Hello Bob!
    Bob-->>Alice: Hi Alice!
"""

renderer.render_to_file(sequence, "sequence.svg", theme="forest")
```

### Gantt Chart

```python
import mermaid_it

renderer = mermaid_it.MermaidRenderer()

gantt = """
gantt
    title Project Timeline
    dateFormat YYYY-MM-DD
    section Planning
    Research :a1, 2024-01-01, 7d
    Design   :a2, after a1, 5d
"""

png_data = renderer.render_png(gantt, width=1200, height=400)
with open("gantt.png", "wb") as f:
    f.write(png_data)
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

## License

MIT License