#![deny(clippy::all)]

use mermaid_it::nodejs_bindings::*;

#[napi::module_exports]
fn init(mut exports: napi::JsObject, env: napi::Env) -> napi::Result<()> {
    let class = env.define_class(
        "NodeMermaidRenderer",
        NodeMermaidRenderer::constructor,
        &[
            NodeMermaidRenderer::set_custom_mermaid_descriptor(),
            NodeMermaidRenderer::render_svg_descriptor(),
            NodeMermaidRenderer::render_png_descriptor(),
            NodeMermaidRenderer::render_jpg_descriptor(),
            NodeMermaidRenderer::render_webp_descriptor(),
            NodeMermaidRenderer::render_to_file_descriptor(),
            NodeMermaidRenderer::render_data_url_descriptor(),
        ],
    )?;
    
    exports.set_named_property("NodeMermaidRenderer", class)?;
    Ok(())
}