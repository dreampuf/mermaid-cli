import mermaid_it.*

fun main() {
    println("🧜‍♀️ Mermaid-it Kotlin Example\n")
    
    // Create a renderer
    val renderer = MermaidRenderer()
    println("✅ Renderer created")
    
    // Example 1: Simple flowchart to SVG
    println("\n1. Rendering flowchart to SVG...")
    val flowchart = """
        graph TD
            A[Kotlin Start] --> B{Is it working?}
            B -->|Yes| C[Excellent!]
            B -->|No| D[Debug]
            D --> B
            C --> E[End]
    """.trimIndent()
    
    val svgOptions = RenderOptions(format = "svg")
    val svgContent = renderer.renderToString(flowchart, svgOptions)
    
    java.io.File("kotlin_flowchart.svg").writeText(svgContent)
    println("   ✓ Saved to kotlin_flowchart.svg (${svgContent.length} bytes)")
    
    // Example 2: Sequence diagram to PNG
    println("\n2. Rendering sequence diagram to PNG...")
    val sequence = """
        sequenceDiagram
            participant Android
            participant ViewModel
            participant Repository
            participant API
            Android->>ViewModel: User Action
            ViewModel->>Repository: Request Data
            Repository->>API: HTTP Request
            API-->>Repository: JSON Response
            Repository-->>ViewModel: Domain Model
            ViewModel-->>Android: UI State
    """.trimIndent()
    
    val pngOptions = RenderOptions(
        format = "png",
        width = 1200u,
        height = 800u,
        theme = "dark",
        scale = 2.0f,
        quality = 90u
    )
    
    val pngData = renderer.render(sequence, pngOptions)
    java.io.File("kotlin_sequence.png").writeBytes(pngData)
    println("   ✓ Saved to kotlin_sequence.png (${pngData.size} bytes)")
    
    // Example 3: Class diagram
    println("\n3. Rendering class diagram...")
    val classDiagram = """
        classDiagram
            class Animal {
                <<abstract>>
                +String name
                +Int age
                +eat()
                +sleep()
            }
            class Dog {
                +String breed
                +bark()
                +wagTail()
            }
            class Cat {
                +String color
                +meow()
                +scratch()
            }
            Animal <|-- Dog : inherits
            Animal <|-- Cat : inherits
            Dog ..> Toy : uses
            Cat ..> Toy : uses
            class Toy {
                +String type
                +play()
            }
    """.trimIndent()
    
    val jpgOptions = RenderOptions(
        format = "jpg",
        width = 1000u,
        height = 800u,
        background = "#f0f0f0",
        quality = 85u
    )
    
    renderer.renderToFile(classDiagram, "kotlin_classes.jpg", jpgOptions)
    println("   ✓ Saved to kotlin_classes.jpg")
    
    // Example 4: State diagram for Android Activity lifecycle
    println("\n4. Rendering Android Activity lifecycle...")
    val lifecycle = """
        stateDiagram-v2
            [*] --> Created : onCreate()
            Created --> Started : onStart()
            Started --> Resumed : onResume()
            Resumed --> Paused : onPause()
            Paused --> Resumed : onResume()
            Paused --> Stopped : onStop()
            Stopped --> Started : onRestart()
            Stopped --> Destroyed : onDestroy()
            Destroyed --> [*]
    """.trimIndent()
    
    val webpOptions = RenderOptions(
        format = "webp",
        width = 900u,
        height = 700u,
        theme = "forest",
        quality = 90u
    )
    
    val webpData = renderer.render(lifecycle, webpOptions)
    java.io.File("kotlin_lifecycle.webp").writeBytes(webpData)
    println("   ✓ Saved to kotlin_lifecycle.webp (${webpData.size} bytes)")
    
    // Example 5: Pie chart
    println("\n5. Rendering pie chart...")
    val pieChart = """
        pie title Kotlin Usage Distribution
            "Android Development" : 45
            "Backend (Spring/Ktor)" : 25
            "Multiplatform" : 15
            "Data Science" : 10
            "Other" : 5
    """.trimIndent()
    
    val pieOptions = RenderOptions(
        format = "svg",
        width = 800u,
        height = 600u,
        theme = "neutral"
    )
    
    val pieSvg = renderer.renderToString(pieChart, pieOptions)
    java.io.File("kotlin_pie.svg").writeText(pieSvg)
    println("   ✓ Saved to kotlin_pie.svg")
    
    println("\n✅ All examples completed successfully!")
    println("\nGenerated files:")
    
    val files = listOf(
        "kotlin_flowchart.svg",
        "kotlin_sequence.png",
        "kotlin_classes.jpg",
        "kotlin_lifecycle.webp",
        "kotlin_pie.svg"
    )
    
    files.forEach { filename ->
        val file = java.io.File(filename)
        if (file.exists()) {
            println("  - $filename (${file.length()} bytes)")
        }
    }
}