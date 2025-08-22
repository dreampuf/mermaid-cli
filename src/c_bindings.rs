use crate::uniffi_bindings::{MermaidRenderer, MermaidError, RenderOptions};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
pub enum CMermaidError {
    Ok = 0,
    RenderError = 1,
    InvalidFormat = 2,
    IoError = 3,
}

#[repr(C)]
pub struct CMermaidOptions {
    pub format: *const c_char,
    pub width: u32,
    pub height: u32,
    pub background: *const c_char,
    pub theme: *const c_char,
    pub scale: f32,
    pub quality: u8,
}

/// Create a new MermaidRenderer
#[no_mangle]
pub extern "C" fn mermaid_renderer_new(error: *mut CMermaidError) -> *mut c_void {
    match MermaidRenderer::new() {
        Ok(renderer) => {
            if !error.is_null() {
                unsafe { *error = CMermaidError::Ok };
            }
            Box::into_raw(Box::new(renderer)) as *mut c_void
        }
        Err(_) => {
            if !error.is_null() {
                unsafe { *error = CMermaidError::RenderError };
            }
            ptr::null_mut()
        }
    }
}

/// Free a MermaidRenderer
#[no_mangle]
pub extern "C" fn mermaid_renderer_free(renderer: *mut c_void) {
    if !renderer.is_null() {
        unsafe {
            let _ = Box::from_raw(renderer as *mut MermaidRenderer);
        }
    }
}

/// Render a diagram to bytes
#[no_mangle]
pub extern "C" fn mermaid_render(
    renderer: *mut c_void,
    diagram: *const c_char,
    options: *const CMermaidOptions,
    out_len: *mut usize,
    error: *mut CMermaidError,
) -> *mut u8 {
    if renderer.is_null() || diagram.is_null() || options.is_null() || out_len.is_null() {
        if !error.is_null() {
            unsafe { *error = CMermaidError::RenderError };
        }
        return ptr::null_mut();
    }
    
    let renderer = unsafe { &*(renderer as *const MermaidRenderer) };
    
    let diagram_str = unsafe {
        match CStr::from_ptr(diagram).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => {
                if !error.is_null() {
                    *error = CMermaidError::RenderError;
                }
                return ptr::null_mut();
            }
        }
    };
    
    let opts = unsafe {
        let format = CStr::from_ptr((*options).format)
            .to_str()
            .unwrap_or("svg")
            .to_string();
        let background = CStr::from_ptr((*options).background)
            .to_str()
            .unwrap_or("white")
            .to_string();
        let theme = CStr::from_ptr((*options).theme)
            .to_str()
            .unwrap_or("default")
            .to_string();
        
        RenderOptions {
            format,
            width: (*options).width,
            height: (*options).height,
            background,
            theme,
            scale: (*options).scale,
            quality: (*options).quality,
        }
    };
    
    match renderer.render(diagram_str, opts) {
        Ok(data) => {
            unsafe {
                *out_len = data.len();
                if !error.is_null() {
                    *error = CMermaidError::Ok;
                }
            }
            
            let mut boxed = data.into_boxed_slice();
            let ptr = boxed.as_mut_ptr();
            std::mem::forget(boxed);
            ptr
        }
        Err(e) => {
            if !error.is_null() {
                unsafe {
                    *error = match e {
                        MermaidError::InvalidFormat { .. } => CMermaidError::InvalidFormat,
                        MermaidError::IoError { .. } => CMermaidError::IoError,
                        _ => CMermaidError::RenderError,
                    };
                }
            }
            ptr::null_mut()
        }
    }
}

/// Free bytes returned by mermaid_render
#[no_mangle]
pub extern "C" fn mermaid_free_bytes(bytes: *mut u8, len: usize) {
    if !bytes.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(bytes, len, len);
        }
    }
}

/// Set custom Mermaid.js content
#[no_mangle]
pub extern "C" fn mermaid_set_custom_js(renderer: *mut c_void, js_content: *const c_char) {
    if renderer.is_null() || js_content.is_null() {
        return;
    }
    
    let renderer = unsafe { &*(renderer as *const MermaidRenderer) };
    
    let js_str = unsafe {
        match CStr::from_ptr(js_content).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return,
        }
    };
    
    renderer.set_custom_mermaid(js_str);
}