# Mermaid-it WASM Bindings

WebAssembly bindings for mermaid-it, enabling Mermaid diagram rendering in browsers and Node.js.

## Installation

### For Browser

```bash
npm install mermaid-it-wasm
```

### For Node.js

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
    
    // Render to SVG
    const svg = await renderer.renderSvg(diagram, 800, 600, 'white', 'default', 1.0);
    document.getElementById('output').innerHTML = svg;
    
    // Render to PNG (returns Uint8Array)
    const pngData = await renderer.renderPng(diagram, 1024, 768);
    
    // Create data URL for display
    const dataUrl = await renderer.renderDataUrl(diagram, 'png', 800, 600);
    const img = document.createElement('img');
    img.src = dataUrl;
    document.body.appendChild(img);
}

renderDiagram();
```

### Browser (Script Tag)

```html
<!DOCTYPE html>
<html>
<head>
    <script type="module">
        import init, { WasmMermaidRenderer } from './node_modules/mermaid-it-wasm/mermaid_it_wasm.js';
        
        async function setup() {
            await init();
            window.renderer = new WasmMermaidRenderer();
        }
        
        setup();
    </script>
</head>
<body>
    <div id="output"></div>
    <script>
        async function render() {
            const svg = await window.renderer.renderSvg('graph TD; A-->B;');
            document.getElementById('output').innerHTML = svg;
        }
    </script>
</body>
</html>
```

### Node.js

```javascript
const { WasmMermaidRenderer } = require('mermaid-it-wasm/nodejs');

async function main() {
    const renderer = new WasmMermaidRenderer();
    
    const diagram = `
    sequenceDiagram
        Alice->>Bob: Hello!
        Bob-->>Alice: Hi!
    `;
    
    const svg = await renderer.renderSvg(diagram);
    console.log(svg);
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

##### async renderSvg(diagramCode, width?, height?, background?, theme?, scale?)

Render a diagram to SVG format.

**Parameters:**
- `diagramCode` (string): The Mermaid diagram code
- `width` (number, optional): Width in pixels (default: 800)
- `height` (number, optional): Height in pixels (default: 600)
- `background` (string, optional): Background color (default: "white")
- `theme` (string, optional): Mermaid theme (default: "default")
- `scale` (number, optional): Scale factor (default: 1.0)

**Returns:** Promise<string> - The SVG content

##### async renderPng(diagramCode, width?, height?, background?, theme?, scale?)

Render a diagram to PNG format.

**Returns:** Promise<Uint8Array> - The PNG image data

##### async renderJpg(diagramCode, width?, height?, background?, theme?, scale?, quality?)

Render a diagram to JPEG format.

**Parameters:**
- `quality` (number, optional): JPEG quality 0-100 (default: 90)

**Returns:** Promise<Uint8Array> - The JPEG image data

##### async renderWebp(diagramCode, width?, height?, background?, theme?, scale?, quality?)

Render a diagram to WebP format.

**Parameters:**
- `quality` (number, optional): WebP quality 0-100 (default: 90)

**Returns:** Promise<Uint8Array> - The WebP image data

##### async renderDataUrl(diagramCode, format?, width?, height?, background?, theme?, scale?, quality?)

Render a diagram to a base64 data URL.

**Parameters:**
- `format` (string, optional): Output format ("svg", "png", "jpg", "webp") (default: "svg")

**Returns:** Promise<string> - The data URL

##### async setCustomMermaid(jsContent)

Set custom Mermaid.js content for rendering.

**Parameters:**
- `jsContent` (string): The Mermaid.js JavaScript content

## Examples

### React Component

```jsx
import React, { useEffect, useState } from 'react';
import init, { WasmMermaidRenderer } from 'mermaid-it-wasm';

function MermaidDiagram({ code }) {
    const [svg, setSvg] = useState('');
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
            renderer.renderSvg(code).then(setSvg);
        }
    }, [renderer, code]);
    
    return <div dangerouslySetInnerHTML={{ __html: svg }} />;
}
```

### Vue Component

```vue
<template>
    <div v-html="svg"></div>
</template>

<script>
import init, { WasmMermaidRenderer } from 'mermaid-it-wasm';

export default {
    props: ['code'],
    data() {
        return {
            svg: '',
            renderer: null
        };
    },
    async mounted() {
        await init();
        this.renderer = new WasmMermaidRenderer();
        this.render();
    },
    watch: {
        code() {
            this.render();
        }
    },
    methods: {
        async render() {
            if (this.renderer && this.code) {
                this.svg = await this.renderer.renderSvg(this.code);
            }
        }
    }
};
</script>
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

## License

MIT License