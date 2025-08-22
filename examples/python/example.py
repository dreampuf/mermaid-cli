#!/usr/bin/env python3
"""
Mermaid-it Python Example
Demonstrates rendering Mermaid diagrams using the UniFFI Python bindings.
"""

import sys
import os

# Add the bindings directory to path (for development)
sys.path.insert(0, '../../bindings/python')

import mermaid_it
from pathlib import Path

def main():
    print("🧜‍♀️ Mermaid-it Python Example\n")
    
    # Create a renderer
    renderer = mermaid_it.MermaidRenderer()
    print("✅ Renderer created")
    
    # Example 1: Simple flowchart to SVG
    print("\n1. Rendering flowchart to SVG...")
    flowchart = """
    graph TD
        A[Start] --> B{Is it working?}
        B -->|Yes| C[Great!]
        B -->|No| D[Debug]
        D --> B
        C --> E[End]
    """
    
    svg_options = mermaid_it.RenderOptions(format="svg")
    svg_content = renderer.render_to_string(flowchart, svg_options)
    
    with open("flowchart.svg", "w") as f:
        f.write(svg_content)
    print(f"   ✓ Saved to flowchart.svg ({len(svg_content)} bytes)")
    
    # Example 2: Sequence diagram to PNG
    print("\n2. Rendering sequence diagram to PNG...")
    sequence = """
    sequenceDiagram
        participant Alice
        participant Bob
        participant Charlie
        Alice->>Bob: Hello Bob!
        Bob->>Charlie: Alice says Hi
        Charlie-->>Alice: Hi Alice!
        Note over Alice,Charlie: Conversation established
    """
    
    png_options = mermaid_it.RenderOptions(
        format="png",
        width=1200,
        height=800,
        theme="dark",
        scale=2.0
    )
    
    png_data = renderer.render(sequence, png_options)
    
    with open("sequence.png", "wb") as f:
        f.write(png_data)
    print(f"   ✓ Saved to sequence.png ({len(png_data)} bytes)")
    
    # Example 3: Gantt chart with custom theme
    print("\n3. Rendering Gantt chart...")
    gantt = """
    gantt
        title Project Timeline
        dateFormat YYYY-MM-DD
        section Planning
        Requirements    :done,    req, 2024-01-01, 7d
        Design          :active,  des, after req, 10d
        section Development
        Backend         :         dev1, after des, 15d
        Frontend        :         dev2, after des, 20d
        section Testing
        Unit Tests      :         test1, after dev1, 5d
        Integration     :         test2, after dev2, 7d
        section Deployment
        Staging         :         stage, after test2, 3d
        Production      :         prod, after stage, 1d
    """
    
    # Direct file save with forest theme
    gantt_options = mermaid_it.RenderOptions(
        format="png",
        width=1400,
        height=600,
        theme="forest",
        quality=95
    )
    
    renderer.render_to_file(gantt, "gantt.png", gantt_options)
    print("   ✓ Saved to gantt.png")
    
    # Example 4: Class diagram to JPEG
    print("\n4. Rendering class diagram to JPEG...")
    class_diagram = """
    classDiagram
        class Animal {
            +String name
            +int age
            +void eat()
            +void sleep()
        }
        class Dog {
            +String breed
            +void bark()
            +void wagTail()
        }
        class Cat {
            +String color
            +void meow()
            +void scratch()
        }
        class Bird {
            +double wingspan
            +void fly()
            +void sing()
        }
        Animal <|-- Dog : inherits
        Animal <|-- Cat : inherits
        Animal <|-- Bird : inherits
    """
    
    jpg_options = mermaid_it.RenderOptions(
        format="jpg",
        width=1000,
        height=800,
        background="#f0f0f0",
        quality=85
    )
    
    jpg_data = renderer.render(class_diagram, jpg_options)
    
    with open("classes.jpg", "wb") as f:
        f.write(jpg_data)
    print(f"   ✓ Saved to classes.jpg ({len(jpg_data)} bytes)")
    
    # Example 5: Pie chart to WebP
    print("\n5. Rendering pie chart to WebP...")
    pie_chart = """
    pie title Programming Languages Used
        "Python" : 35
        "JavaScript" : 25
        "Rust" : 20
        "Go" : 10
        "Other" : 10
    """
    
    webp_options = mermaid_it.RenderOptions(
        format="webp",
        width=800,
        height=600,
        theme="neutral",
        quality=90
    )
    
    webp_data = renderer.render(pie_chart, webp_options)
    
    with open("languages.webp", "wb") as f:
        f.write(webp_data)
    print(f"   ✓ Saved to languages.webp ({len(webp_data)} bytes)")
    
    # Example 6: State diagram
    print("\n6. Rendering state diagram...")
    state_diagram = """
    stateDiagram-v2
        [*] --> Idle
        Idle --> Processing : Start Job
        Processing --> Success : Job Complete
        Processing --> Error : Job Failed
        Success --> Idle : Reset
        Error --> Idle : Reset
        Error --> Processing : Retry
        Success --> [*]
    """
    
    state_options = mermaid_it.RenderOptions(
        format="svg",
        width=900,
        height=600,
        theme="default"
    )
    
    svg_state = renderer.render_to_string(state_diagram, state_options)
    
    with open("state.svg", "w") as f:
        f.write(svg_state)
    print(f"   ✓ Saved to state.svg")
    
    print("\n✅ All examples completed successfully!")
    print("\nGenerated files:")
    for file in ["flowchart.svg", "sequence.png", "gantt.png", "classes.jpg", "languages.webp", "state.svg"]:
        if Path(file).exists():
            size = Path(file).stat().st_size
            print(f"  - {file} ({size:,} bytes)")

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        sys.exit(1)