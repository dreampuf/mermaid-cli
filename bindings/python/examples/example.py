#!/usr/bin/env python3
"""
Example usage of mermaid-it Python bindings
"""

import mermaid_it

def main():
    # Create a renderer instance
    renderer = mermaid_it.MermaidRenderer()
    
    # Define a Mermaid diagram
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
    print("SVG rendered successfully!")
    with open("output.svg", "w") as f:
        f.write(svg)
    
    # Render to PNG with custom settings
    png_bytes = renderer.render_png(
        diagram,
        width=1024,
        height=768,
        background="#f0f0f0",
        theme="dark",
        scale=2.0
    )
    print(f"PNG rendered: {len(png_bytes)} bytes")
    with open("output.png", "wb") as f:
        f.write(png_bytes)
    
    # Render directly to file
    renderer.render_to_file(
        diagram,
        "output.jpg",
        format="jpg",
        width=800,
        height=600,
        quality=95
    )
    print("JPEG saved to output.jpg")
    
    # Sequence diagram example
    sequence_diagram = """
    sequenceDiagram
        participant Alice
        participant Bob
        Alice->>Bob: Hello Bob!
        Bob-->>Alice: Hi Alice!
        Alice->>Bob: How are you?
        Bob-->>Alice: I'm good, thanks!
    """
    
    renderer.render_to_file(
        sequence_diagram,
        "sequence.svg",
        format="svg",
        theme="forest"
    )
    print("Sequence diagram saved to sequence.svg")
    
    # Gantt chart example
    gantt_diagram = """
    gantt
        title Project Timeline
        dateFormat YYYY-MM-DD
        section Planning
        Research           :a1, 2024-01-01, 7d
        Design             :a2, after a1, 5d
        section Development
        Backend            :b1, after a2, 10d
        Frontend           :b2, after a2, 12d
        section Testing
        Unit Tests         :c1, after b1, 3d
        Integration Tests  :c2, after b2, 4d
    """
    
    webp_bytes = renderer.render_webp(
        gantt_diagram,
        width=1200,
        height=400,
        quality=85.0
    )
    print(f"WebP rendered: {len(webp_bytes)} bytes")
    with open("gantt.webp", "wb") as f:
        f.write(webp_bytes)

if __name__ == "__main__":
    main()