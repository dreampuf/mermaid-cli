#!/usr/bin/env node
/**
 * Example usage of mermaid-it Node.js bindings
 */

const { MermaidRenderer } = require('../index');
const fs = require('fs').promises;
const path = require('path');

async function main() {
    // Create a renderer instance
    const renderer = new MermaidRenderer();
    
    // Define a Mermaid diagram
    const diagram = `
    graph TD
        A[Start] --> B{Is it working?}
        B -->|Yes| C[Great!]
        B -->|No| D[Debug]
        D --> B
        C --> E[End]
    `;
    
    console.log('Rendering diagrams...\n');
    
    // Render to SVG
    const svg = await renderer.renderSvg(diagram);
    await fs.writeFile('output.svg', svg);
    console.log('✓ SVG rendered to output.svg');
    
    // Render to PNG with custom settings
    const pngBuffer = await renderer.renderPng(diagram, {
        width: 1024,
        height: 768,
        background: '#f0f0f0',
        theme: 'dark',
        scale: 2.0
    });
    await fs.writeFile('output.png', pngBuffer);
    console.log(`✓ PNG rendered to output.png (${pngBuffer.length} bytes)`);
    
    // Render directly to file
    await renderer.renderToFile(
        diagram,
        'output.jpg',
        {
            format: 'jpg',
            width: 800,
            height: 600,
            quality: 95
        }
    );
    console.log('✓ JPEG saved to output.jpg');
    
    // Sequence diagram example
    const sequenceDiagram = `
    sequenceDiagram
        participant Alice
        participant Bob
        Alice->>Bob: Hello Bob!
        Bob-->>Alice: Hi Alice!
        Alice->>Bob: How are you?
        Bob-->>Alice: I'm good, thanks!
    `;
    
    await renderer.renderToFile(
        sequenceDiagram,
        'sequence.svg',
        {
            format: 'svg',
            theme: 'forest'
        }
    );
    console.log('✓ Sequence diagram saved to sequence.svg');
    
    // Gantt chart example
    const ganttDiagram = `
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
    `;
    
    const webpBuffer = await renderer.renderWebp(ganttDiagram, {
        width: 1200,
        height: 400,
        quality: 85.0
    });
    await fs.writeFile('gantt.webp', webpBuffer);
    console.log(`✓ WebP rendered to gantt.webp (${webpBuffer.length} bytes)`);
    
    // Generate data URL for embedding
    const dataUrl = await renderer.renderDataUrl(diagram, {
        format: 'png',
        width: 400,
        height: 300
    });
    console.log(`✓ Data URL generated (${dataUrl.length} characters)`);
    
    // Create an HTML file with embedded diagram
    const html = `
<!DOCTYPE html>
<html>
<head>
    <title>Mermaid Diagram</title>
    <style>
        body { font-family: Arial, sans-serif; padding: 20px; }
        img { border: 1px solid #ccc; border-radius: 4px; }
    </style>
</head>
<body>
    <h1>Embedded Mermaid Diagram</h1>
    <img src="${dataUrl}" alt="Mermaid diagram">
    <p>This diagram is embedded as a data URL.</p>
</body>
</html>
    `;
    await fs.writeFile('embedded.html', html);
    console.log('✓ HTML with embedded diagram saved to embedded.html');
    
    console.log('\n✅ All examples completed successfully!');
}

// Run the example
main().catch(error => {
    console.error('Error:', error);
    process.exit(1);
});