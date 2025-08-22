import Foundation
import MermaidIt

func main() {
    print("🧜‍♀️ Mermaid-it Swift Example\n")
    
    // Create a renderer
    let renderer = MermaidRenderer()
    print("✅ Renderer created")
    
    // Example 1: Simple flowchart to SVG
    print("\n1. Rendering flowchart to SVG...")
    let flowchart = """
        graph TD
            A[Swift Start] --> B{Is it working?}
            B -->|Yes| C[Great!]
            B -->|No| D[Debug with print]
            D --> B
            C --> E[End]
        """
    
    let svgOptions = RenderOptions(format: "svg")
    
    do {
        let svgContent = try renderer.renderToString(diagram: flowchart, options: svgOptions)
        try svgContent.write(toFile: "swift_flowchart.svg", atomically: true, encoding: .utf8)
        print("   ✓ Saved to swift_flowchart.svg (\(svgContent.count) bytes)")
    } catch {
        print("   ❌ Error: \(error)")
    }
    
    // Example 2: Sequence diagram to PNG
    print("\n2. Rendering sequence diagram to PNG...")
    let sequence = """
        sequenceDiagram
            participant iOS
            participant SwiftUI
            participant ViewModel
            participant CoreData
            iOS->>SwiftUI: User Interaction
            SwiftUI->>ViewModel: Update State
            ViewModel->>CoreData: Save Data
            CoreData-->>ViewModel: Confirmation
            ViewModel-->>SwiftUI: New State
            SwiftUI-->>iOS: UI Update
        """
    
    let pngOptions = RenderOptions(
        format: "png",
        width: 1200,
        height: 800,
        background: "white",
        theme: "dark",
        scale: 2.0,
        quality: 90
    )
    
    do {
        let pngData = try renderer.render(diagram: sequence, options: pngOptions)
        let url = URL(fileURLWithPath: "swift_sequence.png")
        try pngData.write(to: url)
        print("   ✓ Saved to swift_sequence.png (\(pngData.count) bytes)")
    } catch {
        print("   ❌ Error: \(error)")
    }
    
    // Example 3: iOS App Architecture
    print("\n3. Rendering iOS app architecture...")
    let architecture = """
        graph TB
            subgraph "Presentation Layer"
                UI[SwiftUI Views]
                VM[ViewModels]
            end
            
            subgraph "Domain Layer"
                UC[Use Cases]
                EN[Entities]
            end
            
            subgraph "Data Layer"
                REPO[Repositories]
                DS[Data Sources]
                CD[Core Data]
                API[Network API]
            end
            
            UI --> VM
            VM --> UC
            UC --> EN
            UC --> REPO
            REPO --> DS
            DS --> CD
            DS --> API
        """
    
    let archOptions = RenderOptions(
        format: "png",
        width: 1400,
        height: 1000,
        background: "#f0f0f0",
        theme: "forest",
        scale: 1.5,
        quality: 95
    )
    
    do {
        try renderer.renderToFile(
            diagram: architecture,
            outputPath: "swift_architecture.png",
            options: archOptions
        )
        print("   ✓ Saved to swift_architecture.png")
    } catch {
        print("   ❌ Error: \(error)")
    }
    
    // Example 4: State machine for app lifecycle
    print("\n4. Rendering app lifecycle state machine...")
    let lifecycle = """
        stateDiagram-v2
            [*] --> NotRunning
            NotRunning --> Inactive : Launch
            Inactive --> Active : Become Active
            Active --> Inactive : Resign Active
            Inactive --> Background : Enter Background
            Background --> Inactive : Enter Foreground
            Background --> Suspended : Suspend
            Suspended --> Background : Resume
            Suspended --> NotRunning : Terminate
            Inactive --> NotRunning : Terminate
        """
    
    let stateOptions = RenderOptions(
        format: "jpg",
        width: 1000,
        height: 800,
        background: "white",
        theme: "neutral",
        scale: 1.0,
        quality: 85
    )
    
    do {
        let jpgData = try renderer.render(diagram: lifecycle, options: stateOptions)
        let url = URL(fileURLWithPath: "swift_lifecycle.jpg")
        try jpgData.write(to: url)
        print("   ✓ Saved to swift_lifecycle.jpg (\(jpgData.count) bytes)")
    } catch {
        print("   ❌ Error: \(error)")
    }
    
    // Example 5: Swift Package Dependencies
    print("\n5. Rendering package dependencies...")
    let dependencies = """
        graph LR
            App[Your App]
            App --> Alamofire[Alamofire]
            App --> SwiftyJSON[SwiftyJSON]
            App --> Kingfisher[Kingfisher]
            App --> SnapKit[SnapKit]
            App --> RxSwift[RxSwift]
            RxSwift --> RxCocoa[RxCocoa]
            App --> CoreData[Core Data]
            App --> Combine[Combine]
        """
    
    let depOptions = RenderOptions(
        format: "webp",
        width: 1200,
        height: 600,
        background: "#ffffff",
        theme: "default",
        scale: 1.0,
        quality: 90
    )
    
    do {
        let webpData = try renderer.render(diagram: dependencies, options: depOptions)
        let url = URL(fileURLWithPath: "swift_dependencies.webp")
        try webpData.write(to: url)
        print("   ✓ Saved to swift_dependencies.webp (\(webpData.count) bytes)")
    } catch {
        print("   ❌ Error: \(error)")
    }
    
    // Example 6: Pie chart
    print("\n6. Rendering pie chart...")
    let pieChart = """
        pie title iOS Development Time Distribution
            "UI Development" : 35
            "Business Logic" : 25
            "Testing" : 20
            "Debugging" : 15
            "Documentation" : 5
        """
    
    let pieOptions = RenderOptions(
        format: "svg",
        width: 800,
        height: 600,
        theme: "default"
    )
    
    do {
        let pieSvg = try renderer.renderToString(diagram: pieChart, options: pieOptions)
        try pieSvg.write(toFile: "swift_pie.svg", atomically: true, encoding: .utf8)
        print("   ✓ Saved to swift_pie.svg")
    } catch {
        print("   ❌ Error: \(error)")
    }
    
    print("\n✅ All examples completed successfully!")
    print("\nGenerated files:")
    
    let files = [
        "swift_flowchart.svg",
        "swift_sequence.png",
        "swift_architecture.png",
        "swift_lifecycle.jpg",
        "swift_dependencies.webp",
        "swift_pie.svg"
    ]
    
    let fileManager = FileManager.default
    for filename in files {
        if fileManager.fileExists(atPath: filename) {
            if let attributes = try? fileManager.attributesOfItem(atPath: filename),
               let fileSize = attributes[.size] as? Int {
                print("  - \(filename) (\(fileSize) bytes)")
            }
        }
    }
}

// Run the examples
main()