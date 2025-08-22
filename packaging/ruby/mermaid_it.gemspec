Gem::Specification.new do |spec|
  spec.name          = "mermaid-it"
  spec.version       = "0.1.0"
  spec.authors       = ["dreampuf"]
  spec.email         = ["soddyque@gmail.com"]

  spec.summary       = "Ruby bindings for mermaid-it - Fast Mermaid diagram rendering"
  spec.description   = "Native Ruby bindings for mermaid-it, a high-performance Mermaid diagram renderer written in Rust. Supports SVG, PNG, JPEG, WebP, and GIF output formats."
  spec.homepage      = "https://github.com/dreampuf/mermaid-it"
  spec.license       = "MIT"
  spec.required_ruby_version = ">= 2.6.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/dreampuf/mermaid-it"
  spec.metadata["changelog_uri"] = "https://github.com/dreampuf/mermaid-it/blob/main/CHANGELOG.md"
  spec.metadata["documentation_uri"] = "https://github.com/dreampuf/mermaid-it/blob/main/UNIFFI_BINDINGS.md"

  # Specify which files should be added to the gem when it is released.
  spec.files = Dir.glob([
    "lib/**/*",
    "ext/**/*",
    "README.md",
    "LICENSE",
    "CHANGELOG.md"
  ])

  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  # Platform-specific native library
  spec.platform = Gem::Platform::CURRENT
  
  # Include the native library based on platform
  case RUBY_PLATFORM
  when /darwin/
    spec.files << "lib/libmermaid_it.dylib"
  when /linux/
    spec.files << "lib/libmermaid_it.so"
  when /mingw|mswin/
    spec.files << "lib/mermaid_it.dll"
  end

  # Development dependencies
  spec.add_development_dependency "bundler", "~> 2.0"
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rspec", "~> 3.0"
  spec.add_development_dependency "rubocop", "~> 1.0"
  spec.add_development_dependency "yard", "~> 0.9"

  # Post-install message
  spec.post_install_message = <<~MSG
    Thanks for installing mermaid-it!
    
    Quick start:
      require 'mermaid_it'
      renderer = MermaidIt::MermaidRenderer.new
      svg = renderer.render_to_string("graph TD; A-->B;", MermaidIt::RenderOptions.new)
    
    For more examples, visit: https://github.com/dreampuf/mermaid-it
  MSG
end