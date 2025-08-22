#!/usr/bin/env ruby
# frozen_string_literal: true

# Mermaid-it Ruby Example
# Demonstrates rendering Mermaid diagrams using the UniFFI Ruby bindings.

# Add the bindings directory to load path (for development)
$LOAD_PATH.unshift('../../bindings/ruby')

require 'mermaid_it'
require 'fileutils'

def main
  puts "🧜‍♀️ Mermaid-it Ruby Example\n\n"
  
  # Create a renderer
  renderer = MermaidIt::MermaidRenderer.new
  puts "✅ Renderer created"
  
  # Example 1: Simple flowchart to SVG
  puts "\n1. Rendering flowchart to SVG..."
  flowchart = <<~MERMAID
    graph TD
        A[Ruby Start] --> B{Is it working?}
        B -->|Yes| C[Awesome!]
        B -->|No| D[Debug with puts]
        D --> B
        C --> E[End]
  MERMAID
  
  svg_options = MermaidIt::RenderOptions.new
  svg_options.format = "svg"
  
  svg_content = renderer.render_to_string(flowchart, svg_options)
  
  File.write("ruby_flowchart.svg", svg_content)
  puts "   ✓ Saved to ruby_flowchart.svg (#{svg_content.bytesize} bytes)"
  
  # Example 2: Sequence diagram to PNG
  puts "\n2. Rendering sequence diagram to PNG..."
  sequence = <<~MERMAID
    sequenceDiagram
        participant Rails
        participant Controller
        participant Model
        participant Database
        Rails->>Controller: HTTP Request
        Controller->>Model: Find User
        Model->>Database: SELECT * FROM users
        Database-->>Model: User Data
        Model-->>Controller: User Object
        Controller-->>Rails: JSON Response
  MERMAID
  
  png_options = MermaidIt::RenderOptions.new
  png_options.format = "png"
  png_options.width = 1200
  png_options.height = 800
  png_options.theme = "dark"
  png_options.scale = 2.0
  
  png_data = renderer.render(sequence, png_options)
  
  File.binwrite("ruby_sequence.png", png_data)
  puts "   ✓ Saved to ruby_sequence.png (#{png_data.bytesize} bytes)"
  
  # Example 3: Git graph
  puts "\n3. Rendering Git graph..."
  git_graph = <<~MERMAID
    gitGraph
        commit
        commit
        branch develop
        checkout develop
        commit
        commit
        checkout main
        merge develop
        commit
        branch feature
        checkout feature
        commit
        commit
        checkout develop
        merge feature
        checkout main
        merge develop
  MERMAID
  
  git_options = MermaidIt::RenderOptions.new
  git_options.format = "png"
  git_options.width = 1000
  git_options.height = 600
  git_options.theme = "forest"
  
  renderer.render_to_file(git_graph, "ruby_git.png", git_options)
  puts "   ✓ Saved to ruby_git.png"
  
  # Example 4: ERD (Entity Relationship Diagram)
  puts "\n4. Rendering ERD..."
  erd = <<~MERMAID
    erDiagram
        USER ||--o{ ORDER : places
        USER {
            int id PK
            string email
            string name
            datetime created_at
        }
        ORDER ||--|{ ORDER_ITEM : contains
        ORDER {
            int id PK
            int user_id FK
            datetime order_date
            string status
        }
        ORDER_ITEM {
            int id PK
            int order_id FK
            int product_id FK
            int quantity
            decimal price
        }
        PRODUCT ||--o{ ORDER_ITEM : "ordered in"
        PRODUCT {
            int id PK
            string name
            decimal price
            int stock
        }
  MERMAID
  
  erd_options = MermaidIt::RenderOptions.new
  erd_options.format = "svg"
  erd_options.width = 1200
  erd_options.height = 800
  erd_options.background = "#f9f9f9"
  
  erd_svg = renderer.render_to_string(erd, erd_options)
  File.write("ruby_erd.svg", erd_svg)
  puts "   ✓ Saved to ruby_erd.svg"
  
  # Example 5: User Journey
  puts "\n5. Rendering User Journey..."
  journey = <<~MERMAID
    journey
        title Ruby Developer's Day
        section Morning
          Wake up: 5: Me
          Check GitHub: 3: Me
          Read Ruby Weekly: 4: Me
        section Coding
          Write Tests: 5: Me
          Implement Feature: 4: Me
          Debug Issue: 2: Me
          Code Review: 4: Me, Team
        section Evening
          Deploy to Production: 3: Me, DevOps
          Monitor Metrics: 3: Me
          Document Changes: 2: Me
  MERMAID
  
  journey_options = MermaidIt::RenderOptions.new
  journey_options.format = "jpg"
  journey_options.width = 1400
  journey_options.height = 600
  journey_options.theme = "neutral"
  journey_options.quality = 90
  
  journey_data = renderer.render(journey, journey_options)
  File.binwrite("ruby_journey.jpg", journey_data)
  puts "   ✓ Saved to ruby_journey.jpg (#{journey_data.bytesize} bytes)"
  
  # Example 6: Mindmap
  puts "\n6. Rendering Mindmap..."
  mindmap = <<~MERMAID
    mindmap
      root((Ruby Ecosystem))
        Web Frameworks
          Rails
          Sinatra
          Hanami
          Roda
        Testing
          RSpec
          Minitest
          Capybara
        Databases
          ActiveRecord
          Sequel
          ROM
        Background Jobs
          Sidekiq
          Resque
          DelayedJob
        DevOps
          Capistrano
          Docker
          Kubernetes
  MERMAID
  
  mindmap_options = MermaidIt::RenderOptions.new
  mindmap_options.format = "webp"
  mindmap_options.width = 1000
  mindmap_options.height = 1000
  mindmap_options.quality = 85
  
  mindmap_data = renderer.render(mindmap, mindmap_options)
  File.binwrite("ruby_mindmap.webp", mindmap_data)
  puts "   ✓ Saved to ruby_mindmap.webp (#{mindmap_data.bytesize} bytes)"
  
  puts "\n✅ All examples completed successfully!"
  puts "\nGenerated files:"
  
  files = [
    "ruby_flowchart.svg",
    "ruby_sequence.png", 
    "ruby_git.png",
    "ruby_erd.svg",
    "ruby_journey.jpg",
    "ruby_mindmap.webp"
  ]
  
  files.each do |file|
    if File.exist?(file)
      size = File.size(file)
      puts "  - #{file} (#{size.to_s.reverse.gsub(/(\d{3})(?=\d)/, '\\1,').reverse} bytes)"
    end
  end
  
rescue StandardError => e
  puts "❌ Error: #{e.message}"
  puts e.backtrace
  exit 1
end

# Run the examples
main if __FILE__ == $0