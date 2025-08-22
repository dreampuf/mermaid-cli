.PHONY: all clean build-cli build-uniffi build-wasm generate-bindings test

# Default target
all: build-cli build-uniffi build-wasm

# Build the CLI tool
build-cli:
	@echo "Building CLI tool..."
	cargo build --release --features cli

# Build library with UniFFI support
build-uniffi:
	@echo "Building library with UniFFI support..."
	cargo build --release --features uniffi-bindings --lib

# Build WASM bindings
build-wasm:
	@echo "Building WASM bindings..."
	cd bindings/wasm && wasm-pack build --target web --out-dir pkg
	cd bindings/wasm && wasm-pack build --target nodejs --out-dir pkg-node
	cd bindings/wasm && wasm-pack build --target bundler --out-dir pkg-bundler

# Generate all language bindings
generate-bindings: build-uniffi
	@echo "Generating language bindings..."
	@bash generate_bindings.sh

# Generate specific language bindings
generate-kotlin: build-uniffi
	cargo run --bin uniffi-bindgen --features uniffi-bindings -- kotlin

generate-swift: build-uniffi
	cargo run --bin uniffi-bindgen --features uniffi-bindings -- swift

generate-python: build-uniffi
	cargo run --bin uniffi-bindgen --features uniffi-bindings -- python

generate-ruby: build-uniffi
	cargo run --bin uniffi-bindgen --features uniffi-bindings -- ruby

generate-go: build-uniffi
	@echo "Go bindings are already provided in bindings/go/"
	@echo "Build the Rust library with: cargo build --release --features uniffi-bindings"
	@echo "Then use the Go package with CGO"

# Development builds
dev-cli:
	cargo build --features cli

dev-uniffi:
	cargo build --features uniffi-bindings --lib

dev-wasm:
	cd bindings/wasm && wasm-pack build --dev --target web --out-dir pkg

# Testing
test:
	@echo "Running tests..."
	cargo test --all-features
	cd bindings/wasm && wasm-pack test --node || true

# Clean build artifacts
clean:
	cargo clean
	rm -rf bindings/kotlin
	rm -rf bindings/swift
	rm -rf bindings/python
	rm -rf bindings/ruby
	rm -rf bindings/java
	rm -rf bindings/wasm/target
	rm -rf bindings/wasm/pkg
	rm -rf bindings/wasm/pkg-node
	rm -rf bindings/wasm/pkg-bundler

# Install dependencies
install-deps:
	@echo "Installing build dependencies..."
	@command -v wasm-pack >/dev/null 2>&1 || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	@echo "UniFFI is included as a Rust dependency"

# Build documentation
docs:
	cargo doc --no-deps --open

# Format code
fmt:
	cargo fmt --all

# Lint code
lint:
	cargo clippy --all-features -- -D warnings

# Build and publish to npm for WASM
publish-wasm:
	cd bindings/wasm && wasm-pack publish

# Examples of using the generated bindings
example-python:
	@echo "Python example:"
	@echo "  import mermaid_it"
	@echo "  renderer = mermaid_it.MermaidRenderer()"
	@echo "  svg = renderer.render_to_string('graph TD; A-->B;', mermaid_it.RenderOptions())"

example-ruby:
	@echo "Ruby example:"
	@echo "  require 'mermaid_it'"
	@echo "  renderer = MermaidIt::MermaidRenderer.new"
	@echo "  svg = renderer.render_to_string('graph TD; A-->B;', MermaidIt::RenderOptions.new)"

example-kotlin:
	@echo "Kotlin example:"
	@echo "  import mermaid_it.*"
	@echo "  val renderer = MermaidRenderer()"
	@echo "  val svg = renderer.renderToString(\"graph TD; A-->B;\", RenderOptions())"

example-swift:
	@echo "Swift example:"
	@echo "  import MermaidIt"
	@echo "  let renderer = MermaidRenderer()"
	@echo "  let svg = try renderer.renderToString(diagram: \"graph TD; A-->B;\", options: RenderOptions())"

example-go:
	@echo "Go example:"
	@echo "  import \"github.com/dreampuf/mermaid-it/bindings/go\""
	@echo "  renderer, _ := mermaid.NewRenderer()"
	@echo "  defer renderer.Close()"
	@echo "  data, _ := renderer.Render(\"graph TD; A-->B;\", mermaid.DefaultOptions())"