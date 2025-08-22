# Mermaid-it Node.js Bindings

Native Node.js bindings for mermaid-it, providing high-performance Mermaid diagram rendering.

## Installation

```bash
npm install mermaid-it
```

## Quick Start

```javascript
const { MermaidRenderer } = require('mermaid-it');

async function main() {
    // Create a renderer
    const renderer = new MermaidRenderer();
    
    // Define your diagram
    const diagram = `
    graph TD
        A[Start] --> B{Is it working?}
        B -->|Yes| C[Great!]
        B -->|No| D[Debug]
        D --> B
        C --> E[End]
    `;
    
    // Render to SVG
    const svg = await renderer.renderSvg(diagram);
    console.log(svg);
    
    // Render to PNG
    const pngBuffer = await renderer.renderPng(diagram, {
        width: 1024,
        height: 768,
        background: '#f0f0f0',
        theme: 'dark',
        scale: 2.0
    });
    
    // Save to file
    const fs = require('fs').promises;
    await fs.writeFile('output.png', pngBuffer);
}

main();
```

## API Reference

### MermaidRenderer

The main class for rendering Mermaid diagrams.

#### Constructor

```javascript
const renderer = new MermaidRenderer();
```

#### Methods

##### async renderSvg(diagramCode, options?)

Render a diagram to SVG format.

**Parameters:**
- `diagramCode` (string): The Mermaid diagram code
- `options` (object, optional):
  - `width` (number): Width in pixels (default: 800)
  - `height` (number): Height in pixels (default: 600)
  - `background` (string): Background color (default: "white")
  - `theme` (string): Mermaid theme (default: "default")
  - `scale` (number): Scale factor (default: 1.0)

**Returns:** Promise<string> - The SVG content

##### async renderPng(diagramCode, options?)

Render a diagram to PNG format.

**Returns:** Promise<Buffer> - The PNG image data

##### async renderJpg(diagramCode, options?)

Render a diagram to JPEG format.

**Parameters:**
- `options.quality` (number): JPEG quality 0-100 (default: 90)

**Returns:** Promise<Buffer> - The JPEG image data

##### async renderWebp(diagramCode, options?)

Render a diagram to WebP format.

**Parameters:**
- `options.quality` (number): WebP quality 0-100 (default: 90)

**Returns:** Promise<Buffer> - The WebP image data

##### async renderToFile(diagramCode, outputPath, options?)

Render a diagram directly to a file.

**Parameters:**
- `outputPath` (string): Path to save the file
- `options.format` (string): Output format ("svg", "png", "jpg", "webp")

##### async renderDataUrl(diagramCode, options?)

Render a diagram to a base64 data URL.

**Returns:** Promise<string> - The data URL

##### async setCustomMermaid(jsContent)

Set custom Mermaid.js content for rendering.

## Examples

### Express.js Server

```javascript
const express = require('express');
const { MermaidRenderer } = require('mermaid-it');

const app = express();
const renderer = new MermaidRenderer();

app.use(express.json());

app.post('/render', async (req, res) => {
    const { diagram, format = 'svg', ...options } = req.body;
    
    try {
        if (format === 'svg') {
            const svg = await renderer.renderSvg(diagram, options);
            res.type('image/svg+xml').send(svg);
        } else if (format === 'png') {
            const png = await renderer.renderPng(diagram, options);
            res.type('image/png').send(png);
        } else {
            res.status(400).json({ error: 'Unsupported format' });
        }
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.listen(3000, () => {
    console.log('Mermaid renderer service running on port 3000');
});
```

### CLI Tool

```javascript
#!/usr/bin/env node
const { MermaidRenderer } = require('mermaid-it');
const fs = require('fs').promises;

async function cli() {
    const [,, input, output] = process.argv;
    
    if (!input || !output) {
        console.error('Usage: mermaid-render <input.mmd> <output.svg>');
        process.exit(1);
    }
    
    const renderer = new MermaidRenderer();
    const diagram = await fs.readFile(input, 'utf8');
    
    const format = output.split('.').pop();
    await renderer.renderToFile(diagram, output, { format });
    
    console.log(`Rendered ${input} to ${output}`);
}

cli().catch(console.error);
```

### Batch Processing

```javascript
const { MermaidRenderer } = require('mermaid-it');
const fs = require('fs').promises;
const path = require('path');

async function batchRender(inputDir, outputDir) {
    const renderer = new MermaidRenderer();
    const files = await fs.readdir(inputDir);
    
    for (const file of files) {
        if (!file.endsWith('.mmd')) continue;
        
        const input = path.join(inputDir, file);
        const output = path.join(outputDir, file.replace('.mmd', '.svg'));
        
        const diagram = await fs.readFile(input, 'utf8');
        await renderer.renderToFile(diagram, output);
        
        console.log(`✓ ${file}`);
    }
}

batchRender('./diagrams', './output');
```

### TypeScript Support

```typescript
import { MermaidRenderer, RenderOptions } from 'mermaid-it';

const renderer = new MermaidRenderer();

const options: RenderOptions = {
    width: 1024,
    height: 768,
    theme: 'dark',
    scale: 2.0
};

async function render(code: string): Promise<Buffer> {
    return await renderer.renderPng(code, options);
}
```

## Performance

The native Node.js bindings provide significant performance improvements over JavaScript-based solutions:

- **10-50x faster** rendering compared to puppeteer-based solutions
- **Lower memory usage** with efficient Rust implementation
- **Parallel rendering** support for batch operations
- **No browser overhead** - runs directly in Node.js

## Supported Platforms

Pre-built binaries are available for:

- **Windows**: x64
- **macOS**: x64, ARM64 (Apple Silicon)
- **Linux**: x64 (glibc and musl), ARM64

## Building from Source

```bash
# Install dependencies
npm install

# Build native module
npm run build

# Run tests
npm test
```

## License

MIT License