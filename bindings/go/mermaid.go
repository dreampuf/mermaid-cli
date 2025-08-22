package mermaid

// #cgo LDFLAGS: -L../../target/release -lmermaid_it
// #include "mermaid_it.h"
// #include <stdlib.h>
import "C"
import (
	"fmt"
	"unsafe"
)

// Format constants
const (
	FormatSVG  = "svg"
	FormatPNG  = "png"
	FormatJPG  = "jpg"
	FormatJPEG = "jpeg"
	FormatWebP = "webp"
	FormatGIF  = "gif"
)

// Theme constants
const (
	ThemeDefault = "default"
	ThemeDark    = "dark"
	ThemeForest  = "forest"
	ThemeNeutral = "neutral"
)

// RenderOptions contains options for rendering diagrams
type RenderOptions struct {
	Format     string
	Width      uint32
	Height     uint32
	Background string
	Theme      string
	Scale      float32
	Quality    uint8
}

// DefaultOptions returns default render options
func DefaultOptions() RenderOptions {
	return RenderOptions{
		Format:     FormatSVG,
		Width:      800,
		Height:     600,
		Background: "white",
		Theme:      ThemeDefault,
		Scale:      1.0,
		Quality:    90,
	}
}

// Renderer is a Mermaid diagram renderer
type Renderer struct {
	ptr unsafe.Pointer
}

// NewRenderer creates a new Mermaid renderer
func NewRenderer() (*Renderer, error) {
	var errCode C.mermaid_error_t
	ptr := C.mermaid_renderer_new(&errCode)
	
	if ptr == nil {
		return nil, fmt.Errorf("failed to create renderer: error code %d", errCode)
	}
	
	return &Renderer{ptr: ptr}, nil
}

// Close frees the renderer resources
func (r *Renderer) Close() {
	if r.ptr != nil {
		C.mermaid_renderer_free(r.ptr)
		r.ptr = nil
	}
}

// Render renders a Mermaid diagram with the given options
func (r *Renderer) Render(diagram string, options RenderOptions) ([]byte, error) {
	if r.ptr == nil {
		return nil, fmt.Errorf("renderer is closed")
	}
	
	cDiagram := C.CString(diagram)
	defer C.free(unsafe.Pointer(cDiagram))
	
	cFormat := C.CString(options.Format)
	defer C.free(unsafe.Pointer(cFormat))
	
	cBackground := C.CString(options.Background)
	defer C.free(unsafe.Pointer(cBackground))
	
	cTheme := C.CString(options.Theme)
	defer C.free(unsafe.Pointer(cTheme))
	
	cOptions := C.mermaid_options_t{
		format:     cFormat,
		width:      C.uint32_t(options.Width),
		height:     C.uint32_t(options.Height),
		background: cBackground,
		theme:      cTheme,
		scale:      C.float(options.Scale),
		quality:    C.uint8_t(options.Quality),
	}
	
	var outLen C.size_t
	var errCode C.mermaid_error_t
	
	dataPtr := C.mermaid_render(
		r.ptr,
		cDiagram,
		&cOptions,
		&outLen,
		&errCode,
	)
	
	if dataPtr == nil {
		return nil, fmt.Errorf("render failed: error code %d", errCode)
	}
	
	// Copy the data to a Go slice
	data := C.GoBytes(unsafe.Pointer(dataPtr), C.int(outLen))
	
	// Free the C memory
	C.mermaid_free_bytes(dataPtr)
	
	return data, nil
}

// RenderToString renders a diagram and returns it as a string (for SVG)
func (r *Renderer) RenderToString(diagram string, options RenderOptions) (string, error) {
	data, err := r.Render(diagram, options)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

// SetCustomMermaidJS sets custom Mermaid.js content
func (r *Renderer) SetCustomMermaidJS(jsContent string) {
	if r.ptr == nil {
		return
	}
	
	cJS := C.CString(jsContent)
	defer C.free(unsafe.Pointer(cJS))
	
	C.mermaid_set_custom_js(r.ptr, cJS)
}

// QuickRender is a convenience function for simple rendering
func QuickRender(diagram string, format string) ([]byte, error) {
	renderer, err := NewRenderer()
	if err != nil {
		return nil, err
	}
	defer renderer.Close()
	
	options := DefaultOptions()
	options.Format = format
	
	return renderer.Render(diagram, options)
}