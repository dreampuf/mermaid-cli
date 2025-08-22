use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions::PyRuntimeError;
use crate::MermaidIt;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Python wrapper for MermaidIt
#[pyclass(name = "MermaidRenderer")]
struct PyMermaidRenderer {
    inner: Arc<Mutex<MermaidIt>>,
    runtime: Arc<tokio::runtime::Runtime>,
}

#[pymethods]
impl PyMermaidRenderer {
    /// Create a new MermaidRenderer instance
    #[new]
    fn new() -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        
        let renderer = MermaidIt::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create renderer: {}", e)))?;
        
        Ok(Self {
            inner: Arc::new(Mutex::new(renderer)),
            runtime: Arc::new(runtime),
        })
    }
    
    /// Set custom Mermaid.js content
    fn set_custom_mermaid(&self, js_content: String) -> PyResult<()> {
        let inner = self.inner.clone();
        self.runtime.block_on(async move {
            let mut renderer = inner.lock().await;
            renderer.set_custom_mermaid(js_content);
            Ok(())
        })
    }
    
    /// Render diagram to SVG
    #[pyo3(signature = (diagram_code, width=800, height=600, background="white", theme="default", scale=1.0))]
    fn render_svg(
        &self,
        diagram_code: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
    ) -> PyResult<String> {
        let inner = self.inner.clone();
        let diagram = diagram_code.to_string();
        let bg = background.to_string();
        let th = theme.to_string();
        
        self.runtime.block_on(async move {
            let mut renderer = inner.lock().await;
            renderer.render_svg(&diagram, width, height, &bg, &th, scale)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))
        })
    }
    
    /// Render diagram to PNG bytes
    #[pyo3(signature = (diagram_code, width=800, height=600, background="white", theme="default", scale=1.0))]
    fn render_png<'py>(
        &self,
        py: Python<'py>,
        diagram_code: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let inner = self.inner.clone();
        let diagram = diagram_code.to_string();
        let bg = background.to_string();
        let th = theme.to_string();
        
        let png_data = self.runtime.block_on(async move {
            let mut renderer = inner.lock().await;
            renderer.render_png(&diagram, width, height, &bg, &th, scale)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))
        })?;
        
        Ok(PyBytes::new_bound(py, &png_data))
    }
    
    /// Render diagram to JPEG bytes
    #[pyo3(signature = (diagram_code, width=800, height=600, background="white", theme="default", scale=1.0, quality=90))]
    fn render_jpg<'py>(
        &self,
        py: Python<'py>,
        diagram_code: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
        quality: u8,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let inner = self.inner.clone();
        let diagram = diagram_code.to_string();
        let bg = background.to_string();
        let th = theme.to_string();
        
        let jpg_data = self.runtime.block_on(async move {
            let mut renderer = inner.lock().await;
            renderer.render_jpg(&diagram, width, height, &bg, &th, scale, quality)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))
        })?;
        
        Ok(PyBytes::new_bound(py, &jpg_data))
    }
    
    /// Render diagram to WebP bytes
    #[pyo3(signature = (diagram_code, width=800, height=600, background="white", theme="default", scale=1.0, quality=90.0))]
    fn render_webp<'py>(
        &self,
        py: Python<'py>,
        diagram_code: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
        quality: f32,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let inner = self.inner.clone();
        let diagram = diagram_code.to_string();
        let bg = background.to_string();
        let th = theme.to_string();
        
        let webp_data = self.runtime.block_on(async move {
            let mut renderer = inner.lock().await;
            renderer.render_webp(&diagram, width, height, &bg, &th, scale, quality)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))
        })?;
        
        Ok(PyBytes::new_bound(py, &webp_data))
    }
    
    /// Render diagram to file
    #[pyo3(signature = (diagram_code, output_path, format="svg", width=800, height=600, background="white", theme="default", scale=1.0, quality=90))]
    fn render_to_file(
        &self,
        diagram_code: &str,
        output_path: &str,
        format: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
        quality: u8,
    ) -> PyResult<()> {
        let inner = self.inner.clone();
        let diagram = diagram_code.to_string();
        let bg = background.to_string();
        let th = theme.to_string();
        let path = output_path.to_string();
        let fmt = format.to_string();
        
        self.runtime.block_on(async move {
            let mut renderer = inner.lock().await;
            
            let data = match fmt.as_str() {
                "svg" => {
                    let svg = renderer.render_svg(&diagram, width, height, &bg, &th, scale)
                        .await
                        .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))?;
                    svg.into_bytes()
                },
                "png" => {
                    renderer.render_png(&diagram, width, height, &bg, &th, scale)
                        .await
                        .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))?
                },
                "jpg" | "jpeg" => {
                    renderer.render_jpg(&diagram, width, height, &bg, &th, scale, quality)
                        .await
                        .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))?
                },
                "webp" => {
                    renderer.render_webp(&diagram, width, height, &bg, &th, scale, quality as f32)
                        .await
                        .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))?
                },
                _ => return Err(PyRuntimeError::new_err(format!("Unsupported format: {}", fmt))),
            };
            
            std::fs::write(&path, data)
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to write file: {}", e)))?;
            
            Ok(())
        })
    }
}

/// Python module initialization
#[pymodule]
fn mermaid_it(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMermaidRenderer>()?;
    Ok(())
}