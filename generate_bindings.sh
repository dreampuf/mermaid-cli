#!/bin/bash

# Generate bindings for all supported languages

echo "Building Rust library with UniFFI support..."
cargo build --release --features uniffi-bindings

echo ""
echo "Generating language bindings..."

# UniFFI-supported languages
for lang in kotlin swift python ruby; do
    echo "Generating $lang bindings..."
    cargo run --bin uniffi-bindgen --features uniffi-bindings -- $lang
done

# Go requires special handling (C bindings)
echo ""
echo "Generating Go bindings..."
mkdir -p bindings/go

# Generate C header for Go
echo "Generating C header for Go..."
cat > bindings/go/mermaid_it.h << 'EOF'
#ifndef MERMAID_IT_H
#define MERMAID_IT_H

#include <stdint.h>

// Opaque pointer to MermaidRenderer
typedef void* mermaid_renderer_t;

// Error codes
typedef enum {
    MERMAID_OK = 0,
    MERMAID_ERROR_RENDER = 1,
    MERMAID_ERROR_INVALID_FORMAT = 2,
    MERMAID_ERROR_IO = 3,
} mermaid_error_t;

// Render options
typedef struct {
    const char* format;
    uint32_t width;
    uint32_t height;
    const char* background;
    const char* theme;
    float scale;
    uint8_t quality;
} mermaid_options_t;

// Create a new renderer
mermaid_renderer_t mermaid_renderer_new(mermaid_error_t* error);

// Free a renderer
void mermaid_renderer_free(mermaid_renderer_t renderer);

// Render diagram to bytes
// Returns: pointer to data (must be freed with mermaid_free_bytes)
// Sets *out_len to the length of the data
uint8_t* mermaid_render(
    mermaid_renderer_t renderer,
    const char* diagram,
    const mermaid_options_t* options,
    size_t* out_len,
    mermaid_error_t* error
);

// Free bytes returned by mermaid_render
void mermaid_free_bytes(uint8_t* bytes);

// Set custom Mermaid.js content
void mermaid_set_custom_js(mermaid_renderer_t renderer, const char* js_content);

#endif // MERMAID_IT_H
EOF

# Java uses Kotlin bindings via JNI
echo ""
echo "Setting up Java bindings (via Kotlin)..."
mkdir -p bindings/java
echo "Java will use the Kotlin bindings through JNI. See bindings/kotlin for the generated code."

echo ""
echo "✅ All bindings generated successfully!"
echo ""
echo "Generated bindings locations:"
echo "  - Kotlin: bindings/kotlin/"
echo "  - Swift:  bindings/swift/"
echo "  - Python: bindings/python/"
echo "  - Ruby:   bindings/ruby/"
echo "  - Go:     bindings/go/ (C header provided, Go wrapper needed)"
echo "  - Java:   Use Kotlin bindings via JNI"