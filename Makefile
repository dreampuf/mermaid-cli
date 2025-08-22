.PHONY: all clean build-cli build-python build-wasm build-nodejs test

# Default target
all: build-cli build-python build-wasm build-nodejs

# Build the CLI tool
build-cli:
	@echo "Building CLI tool..."
	cargo build --release --features cli

# Build Python bindings
build-python:
	@echo "Building Python bindings..."
	cd bindings/python && maturin build --release

# Build WASM bindings
build-wasm:
	@echo "Building WASM bindings..."
	cd bindings/wasm && wasm-pack build --target web --out-dir pkg

# Build Node.js bindings
build-nodejs:
	@echo "Building Node.js bindings..."
	cd bindings/nodejs && npm install && npm run build

# Development builds
dev-cli:
	cargo build --features cli

dev-python:
	cd bindings/python && maturin develop

dev-wasm:
	cd bindings/wasm && wasm-pack build --dev --target web --out-dir pkg

dev-nodejs:
	cd bindings/nodejs && npm install && npm run build:debug

# Testing
test:
	@echo "Running tests..."
	cargo test --all-features
	cd bindings/python && python -m pytest tests/ || true
	cd bindings/nodejs && npm test || true

# Clean build artifacts
clean:
	cargo clean
	rm -rf bindings/python/target
	rm -rf bindings/python/dist
	rm -rf bindings/wasm/target
	rm -rf bindings/wasm/pkg
	rm -rf bindings/nodejs/target
	rm -rf bindings/nodejs/*.node

# Install dependencies
install-deps:
	@echo "Installing build dependencies..."
	@command -v maturin >/dev/null 2>&1 || pip install maturin
	@command -v wasm-pack >/dev/null 2>&1 || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	@cd bindings/nodejs && npm install

# Build documentation
docs:
	cargo doc --no-deps --open

# Format code
fmt:
	cargo fmt --all

# Lint code
lint:
	cargo clippy --all-features -- -D warnings

# Build and publish Python package
publish-python:
	cd bindings/python && maturin publish

# Build and publish npm package
publish-nodejs:
	cd bindings/nodejs && npm publish

# Build and publish to npm for WASM
publish-wasm:
	cd bindings/wasm && wasm-pack publish