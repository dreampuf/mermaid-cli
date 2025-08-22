/**
 * Node.js bindings for mermaid-it
 */

export interface RenderOptions {
  width?: number;
  height?: number;
  background?: string;
  theme?: string;
  scale?: number;
}

export interface RenderFileOptions extends RenderOptions {
  format?: 'svg' | 'png' | 'jpg' | 'jpeg' | 'webp';
  quality?: number;
}

/**
 * Mermaid diagram renderer using native Rust implementation
 */
export class MermaidRenderer {
  constructor();
  
  /**
   * Set custom Mermaid.js content
   * @param jsContent - The Mermaid.js JavaScript content
   */
  setCustomMermaid(jsContent: string): Promise<void>;
  
  /**
   * Render a Mermaid diagram to SVG
   * @param diagramCode - The Mermaid diagram code
   * @param options - Rendering options
   * @returns The rendered SVG as a string
   */
  renderSvg(diagramCode: string, options?: RenderOptions): Promise<string>;
  
  /**
   * Render a Mermaid diagram to PNG
   * @param diagramCode - The Mermaid diagram code
   * @param options - Rendering options
   * @returns The rendered PNG as a Buffer
   */
  renderPng(diagramCode: string, options?: RenderOptions): Promise<Buffer>;
  
  /**
   * Render a Mermaid diagram to JPEG
   * @param diagramCode - The Mermaid diagram code
   * @param options - Rendering options with quality (0-100)
   * @returns The rendered JPEG as a Buffer
   */
  renderJpg(diagramCode: string, options?: RenderOptions & { quality?: number }): Promise<Buffer>;
  
  /**
   * Render a Mermaid diagram to WebP
   * @param diagramCode - The Mermaid diagram code
   * @param options - Rendering options with quality (0-100)
   * @returns The rendered WebP as a Buffer
   */
  renderWebp(diagramCode: string, options?: RenderOptions & { quality?: number }): Promise<Buffer>;
  
  /**
   * Render a Mermaid diagram to a file
   * @param diagramCode - The Mermaid diagram code
   * @param outputPath - The output file path
   * @param options - Rendering options including format
   */
  renderToFile(diagramCode: string, outputPath: string, options?: RenderFileOptions): Promise<void>;
  
  /**
   * Render a Mermaid diagram to a base64 data URL
   * @param diagramCode - The Mermaid diagram code
   * @param options - Rendering options including format
   * @returns The data URL string
   */
  renderDataUrl(diagramCode: string, options?: RenderFileOptions): Promise<string>;
}

export { MermaidRenderer as NodeMermaidRenderer };