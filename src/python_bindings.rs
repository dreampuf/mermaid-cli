use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions::PyRuntimeError;
use crate::{MermaidIt, OutputFormat, RenderOptions};
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
    
    /// Render diagram to specified format
    #[pyo3(signature = (diagram_code, format="svg", width=800, height=600, background="white", theme="default", scale=1.0, quality=90))]
    fn render<'py>(
        &self,
        py: Python<'py>,
        diagram_code: &str,
        format: &str,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
        quality: u8,
    ) -> PyResult<PyObject> {
        let output_format = OutputFormat::from_str(format)
            .ok_or_else(|| PyRuntimeError::new_err(format!("Unsupported format: {}", format)))?;
        
        let options = RenderOptions {
            width,
            height,
            background: background.to_string(),
            theme: theme.to_string(),
            scale,
            quality,
        };
        
        let inner = self.inner.clone();
        let diagram = diagram_code.to_string();
        
        let data = self.runtime.block_on(async move {
            let mut renderer = inner.lock().await;
            renderer.render(&diagram, output_format, &options)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))
        })?;
        
        // Return string for SVG, bytes for other formats
        match output_format {
            OutputFormat::Svg => {
                let svg_str = String::from_utf8(data)
                    .map_err(|e| PyRuntimeError::new_err(format!("Invalid UTF-8: {}", e)))?;
                Ok(svg_str.into_py(py))
            },
            _ => Ok(PyBytes::new_bound(py, &data).into()),
        }
    }
    
    /// Render diagram to file
    #[pyo3(signature = (diagram_code, output_path, format=None, width=800, height=600, background="white", theme="default", scale=1.0, quality=90))]
    fn render_to_file(
        &self,
        diagram_code: &str,
        output_path: &str,
        format: Option<&str>,
        width: u32,
        height: u32,
        background: &str,
        theme: &str,
        scale: f32,
        quality: u8,
    ) -> PyResult<()> {
        // Determine format from file extension if not provided
        let fmt = format.unwrap_or_else(|| {
            output_path.rsplit('.').next().unwrap_or("svg")
        });
        
        let output_format = OutputFormat::from_str(fmt)
            .ok_or_else(|| PyRuntimeError::new_err(format!("Unsupported format: {}", fmt)))?;
        
        let options = RenderOptions {
            width,
            height,
            background: background.to_string(),
            theme: theme.to_string(),
            scale,
            quality,
        };
        
        let inner = self.inner.clone();
        let diagram = diagram_code.to_string();
        let path = output_path.to_string();
        
        self.runtime.block_on(async move {
            let mut renderer = inner.lock().await;
            let data = renderer.render(&diagram, output_format, &options)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))?;
            
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