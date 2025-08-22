# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- UniFFI-based language bindings for Python, Ruby, Kotlin, Swift
- C FFI bindings for Go integration
- Unified API across all language bindings
- Comprehensive examples for Python, Ruby, and Go
- CI/CD workflow for multi-language builds and publishing
- Support for WebP and GIF output formats
- Configurable rendering options (width, height, theme, scale, quality)
- WASM bindings for JavaScript/TypeScript (browser and Node.js)

### Changed
- Migrated from individual binding implementations to UniFFI
- Consolidated rendering API to single `render` method with format parameter
- Improved error handling with typed errors across all bindings
- Updated build system to support multiple language targets

### Removed
- Native Node.js bindings (replaced with WASM)
- Duplicate API methods for different formats

## [0.1.0] - 2024-01-XX (Initial Release)

### Added
- Core CLI tool for rendering Mermaid diagrams
- Support for SVG, PNG, JPEG output formats
- Embedded Mermaid.js using deno_core
- Custom Mermaid.js file support
- Configurable themes and styling options
- Basic Python bindings via PyO3
- Basic WASM bindings

[Unreleased]: https://github.com/dreampuf/mermaid-it/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/dreampuf/mermaid-it/releases/tag/v0.1.0