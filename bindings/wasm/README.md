# Mermaid-it WASM Bindings

WebAssembly bindings for mermaid-it, enabling Mermaid diagram rendering in browsers and Node.js.

## Installation

```bash
npm install mermaid-it-wasm
```

## Usage

### Browser (ES Modules)

```javascript
import init, { WasmMermaidRenderer } from 'mermaid-it-wasm';

async function renderDiagram() {
    // Initialize the WASM module
    await init();
    
    // Create a renderer
    const renderer = new WasmMermaidRenderer();
    
    // Define your diagram
    const diagram = `
    graph TD
        A[Start] --> B{Is it working?}
        B -->|Yes| C[Great!]
        B -->|No| D[Debug]
    `;
    
    // Render to SVG (default)
    const svg = await renderer.render(diagram, {
        format: 'svg',
        width: 800,
        height: 600,
        theme: 'default'
    });
    document.getElementById('output').innerHTML = svg;
    
    // Render to PNG (returns Uint8Array)
    const pngData = await renderer.render(diagram, {
        format: 'png',
        width: 1024,
        height: 768,
        scale: 2.0
    });
    
    // Create data URL for display
    const dataUrl = await renderer.renderDataUrl(diagram, {
        format: 'png',
        width: 800,
        height: 600
    });
    const img = document.createElement('img');
    img.src = dataUrl;
    document.body.appendChild(img);
}

renderDiagram();
```

### Node.js

```javascript
const { WasmMermaidRenderer } = require('mermaid-it-wasm');

async function main() {
    const renderer = new WasmMermaidRenderer();
    
    const diagram = `
    sequenceDiagram
        Alice->>Bob: Hello!
        Bob-->>Alice: Hi!
    `;
    
    // Render to SVG
    const svg = await renderer.render(diagram, { format: 'svg' });
    console.log(svg);
    
    // Render to PNG
    const pngData = await renderer.render(diagram, { 
        format: 'png',
        width: 1200,
        height: 800 
    });
    // pngData is a Uint8Array
    
    const fs = require('fs');
    fs.writeFileSync('diagram.png', Buffer.from(pngData));
}

main();
```

## API Reference

### WasmMermaidRenderer

#### Constructor

```javascript
const renderer = new WasmMermaidRenderer();
```

#### Methods

##### async render(diagramCode, options?)

Render a diagram to the specified format.

**Parameters:**
- `diagramCode` (string): The Mermaid diagram code
- `options` (object, optional): Rendering options
  - `format` (string): Output format - "svg", "png", "jpg", "jpeg", "webp", "gif" (default: "svg")
  - `width` (number): Width in pixels (default: 800)
  - `height` (number): Height in pixels (default: 600)
  - `background` (string): Background color (default: "white")
  - `theme` (string): Mermaid theme (default: "default")
  - `scale` (number): Scale factor (default: 1.0)
  - `quality` (number): Quality for JPEG/WebP formats, 0-100 (default: 90)

**Returns:**
- For SVG: `Promise<string>` - The SVG content
- For other formats: `Promise<Uint8Array>` - The image data

##### async renderDataUrl(diagramCode, options?)

Render a diagram to a base64 data URL.

**Parameters:**
- Same as `render()`

**Returns:** `Promise<string>` - The data URL

##### async setCustomMermaid(jsContent)

Set custom Mermaid.js content for rendering.

**Parameters:**
- `jsContent` (string): The Mermaid.js JavaScript content

## Examples

### React Component

```jsx
import React, { useEffect, useState } from 'react';
import init, { WasmMermaidRenderer } from 'mermaid-it-wasm';

function MermaidDiagram({ code, format = 'svg', ...options }) {
    const [output, setOutput] = useState(null);
    const [renderer, setRenderer] = useState(null);
    
    useEffect(() => {
        async function setup() {
            await init();
            setRenderer(new WasmMermaidRenderer());
        }
        setup();
    }, []);
    
    useEffect(() => {
        if (renderer && code) {
            renderer.render(code, { format, ...options })
                .then(data => {
                    if (format === 'svg') {
                        setOutput(data);
                    } else {
                        // Convert Uint8Array to data URL for images
                        const blob = new Blob([data], { type: `image/${format}` });
                        const url = URL.createObjectURL(blob);
                        setOutput(url);
                    }
                });
        }
    }, [renderer, code, format, options]);
    
    if (format === 'svg') {
        return <div dangerouslySetInnerHTML={{ __html: output }} />;
    } else {
        return output ? <img src={output} alt="Mermaid diagram" /> : null;
    }
}

// Usage
<MermaidDiagram 
    code="graph TD; A-->B;" 
    format="png"
    width={1024}
    height={768}
    theme="dark"
/>
```

### Vue Component

```vue
<template>
    <div v-if="format === 'svg'" v-html="output"></div>
    <img v-else-if="output" :src="output" alt="Mermaid diagram" />
</template>

<script>
import init, { WasmMermaidRenderer } from 'mermaid-it-wasm';

export default {
    props: {
        code: String,
        format: { type: String, default: 'svg' },
        width: { type: Number, default: 800 },
        height: { type: Number, default: 600 },
        theme: { type: String, default: 'default' }
    },
    data() {
        return {
            output: null,
            renderer: null
        };
    },
    async mounted() {
        await init();
        this.renderer = new WasmMermaidRenderer();
        this.render();
    },
    watch: {
        code() { this.render(); },
        format() { this.render(); },
        width() { this.render(); },
        height() { this.render(); },
        theme() { this.render(); }
    },
    methods: {
        async render() {
            if (this.renderer && this.code) {
                const data = await this.renderer.render(this.code, {
                    format: this.format,
                    width: this.width,
                    height: this.height,
                    theme: this.theme
                });
                
                if (this.format === 'svg') {
                    this.output = data;
                } else {
                    // Convert to data URL
                    const blob = new Blob([data], { type: `image/${this.format}` });
                    this.output = URL.createObjectURL(blob);
                }
            }
        }
    }
};
</script>
```

### Express.js Server

```javascript
const express = require('express');
const { WasmMermaidRenderer } = require('mermaid-it-wasm');

const app = express();
app.use(express.json());

let renderer;

// Initialize renderer on startup
(async () => {
    renderer = new WasmMermaidRenderer();
})();

app.post('/render', async (req, res) => {
    const { diagram, format = 'svg', ...options } = req.body;
    
    try {
        const data = await renderer.render(diagram, { format, ...options });
        
        if (format === 'svg') {
            res.type('image/svg+xml').send(data);
        } else {
            // Convert Uint8Array to Buffer for binary formats
            res.type(`image/${format}`).send(Buffer.from(data));
        }
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.listen(3000, () => {
    console.log('Mermaid renderer service running on port 3000');
});
```

## Building from Source

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for web
wasm-pack build --target web --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg-node

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler --out-dir pkg-bundler
```

## Performance

- **Fast**: Near-native performance with WebAssembly
- **Small**: Minimal bundle size with optimized WASM
- **Universal**: Works in browsers, Node.js, and Deno
- **No Dependencies**: Self-contained WASM module

## License

MIT License