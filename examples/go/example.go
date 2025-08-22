package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	
	mermaid "github.com/dreampuf/mermaid-it/bindings/go"
)

func main() {
	fmt.Println("🧜‍♀️ Mermaid-it Go Example\n")
	
	// Create a renderer
	renderer, err := mermaid.NewRenderer()
	if err != nil {
		log.Fatalf("Failed to create renderer: %v", err)
	}
	defer renderer.Close()
	
	fmt.Println("✅ Renderer created")
	
	// Example 1: Simple flowchart to SVG
	fmt.Println("\n1. Rendering flowchart to SVG...")
	flowchart := `
	graph TD
		A[Go Start] --> B{Is it working?}
		B -->|Yes| C[Excellent!]
		B -->|No| D[Check error]
		D --> E[Fix issue]
		E --> B
		C --> F[End]
	`
	
	svgOptions := mermaid.DefaultOptions()
	svgOptions.Format = mermaid.FormatSVG
	
	svgData, err := renderer.Render(flowchart, svgOptions)
	if err != nil {
		log.Fatalf("Failed to render flowchart: %v", err)
	}
	
	err = ioutil.WriteFile("go_flowchart.svg", svgData, 0644)
	if err != nil {
		log.Fatalf("Failed to write SVG: %v", err)
	}
	fmt.Printf("   ✓ Saved to go_flowchart.svg (%d bytes)\n", len(svgData))
	
	// Example 2: Sequence diagram to PNG
	fmt.Println("\n2. Rendering sequence diagram to PNG...")
	sequence := `
	sequenceDiagram
		participant Client
		participant Server
		participant Database
		participant Cache
		Client->>Server: HTTP Request
		Server->>Cache: Check Cache
		Cache-->>Server: Cache Miss
		Server->>Database: Query Data
		Database-->>Server: Return Data
		Server->>Cache: Store in Cache
		Server-->>Client: JSON Response
	`
	
	pngOptions := mermaid.RenderOptions{
		Format:     mermaid.FormatPNG,
		Width:      1200,
		Height:     800,
		Background: "white",
		Theme:      mermaid.ThemeDark,
		Scale:      2.0,
		Quality:    90,
	}
	
	pngData, err := renderer.Render(sequence, pngOptions)
	if err != nil {
		log.Fatalf("Failed to render sequence: %v", err)
	}
	
	err = ioutil.WriteFile("go_sequence.png", pngData, 0644)
	if err != nil {
		log.Fatalf("Failed to write PNG: %v", err)
	}
	fmt.Printf("   ✓ Saved to go_sequence.png (%d bytes)\n", len(pngData))
	
	// Example 3: Architecture diagram
	fmt.Println("\n3. Rendering architecture diagram...")
	architecture := `
	graph TB
		subgraph "Frontend"
			UI[React UI]
			Mobile[Mobile App]
		end
		
		subgraph "API Gateway"
			GW[Kong/Nginx]
		end
		
		subgraph "Microservices"
			Auth[Auth Service]
			User[User Service]
			Order[Order Service]
			Payment[Payment Service]
		end
		
		subgraph "Data Layer"
			PG[(PostgreSQL)]
			Redis[(Redis)]
			Kafka[Apache Kafka]
		end
		
		UI --> GW
		Mobile --> GW
		GW --> Auth
		GW --> User
		GW --> Order
		GW --> Payment
		Auth --> Redis
		User --> PG
		Order --> PG
		Order --> Kafka
		Payment --> Kafka
	`
	
	archOptions := mermaid.RenderOptions{
		Format:     mermaid.FormatPNG,
		Width:      1400,
		Height:     1000,
		Background: "#f0f0f0",
		Theme:      mermaid.ThemeForest,
		Scale:      1.5,
		Quality:    95,
	}
	
	archData, err := renderer.Render(architecture, archOptions)
	if err != nil {
		log.Fatalf("Failed to render architecture: %v", err)
	}
	
	err = ioutil.WriteFile("go_architecture.png", archData, 0644)
	if err != nil {
		log.Fatalf("Failed to write architecture PNG: %v", err)
	}
	fmt.Println("   ✓ Saved to go_architecture.png")
	
	// Example 4: State machine
	fmt.Println("\n4. Rendering state machine...")
	stateMachine := `
	stateDiagram-v2
		[*] --> Idle
		Idle --> Connecting : Connect
		Connecting --> Connected : Success
		Connecting --> Error : Failure
		Connected --> Transmitting : Send Data
		Transmitting --> Connected : Complete
		Connected --> Disconnecting : Disconnect
		Disconnecting --> Idle : Complete
		Error --> Idle : Reset
		Connected --> Error : Connection Lost
	`
	
	stateOptions := mermaid.RenderOptions{
		Format:     mermaid.FormatJPEG,
		Width:      1000,
		Height:     800,
		Background: "white",
		Theme:      mermaid.ThemeNeutral,
		Scale:      1.0,
		Quality:    85,
	}
	
	stateData, err := renderer.Render(stateMachine, stateOptions)
	if err != nil {
		log.Fatalf("Failed to render state machine: %v", err)
	}
	
	err = ioutil.WriteFile("go_state.jpg", stateData, 0644)
	if err != nil {
		log.Fatalf("Failed to write state JPEG: %v", err)
	}
	fmt.Printf("   ✓ Saved to go_state.jpg (%d bytes)\n", len(stateData))
	
	// Example 5: Deployment pipeline
	fmt.Println("\n5. Rendering deployment pipeline...")
	pipeline := `
	graph LR
		subgraph "Development"
			Dev[Developer] --> Git[Git Push]
		end
		
		subgraph "CI/CD"
			Git --> CI[GitHub Actions]
			CI --> Test[Run Tests]
			Test --> Build[Build Docker Image]
			Build --> Push[Push to Registry]
		end
		
		subgraph "Deployment"
			Push --> Stage[Deploy to Staging]
			Stage --> E2E[E2E Tests]
			E2E --> Prod[Deploy to Production]
		end
		
		subgraph "Monitoring"
			Prod --> Metrics[Prometheus]
			Prod --> Logs[ELK Stack]
			Prod --> Alerts[PagerDuty]
		end
	`
	
	pipelineOptions := mermaid.RenderOptions{
		Format:     mermaid.FormatWebP,
		Width:      1600,
		Height:     600,
		Background: "#ffffff",
		Theme:      mermaid.ThemeDefault,
		Scale:      1.0,
		Quality:    90,
	}
	
	pipelineData, err := renderer.Render(pipeline, pipelineOptions)
	if err != nil {
		log.Fatalf("Failed to render pipeline: %v", err)
	}
	
	err = ioutil.WriteFile("go_pipeline.webp", pipelineData, 0644)
	if err != nil {
		log.Fatalf("Failed to write pipeline WebP: %v", err)
	}
	fmt.Printf("   ✓ Saved to go_pipeline.webp (%d bytes)\n", len(pipelineData))
	
	// Example 6: Quick render helper
	fmt.Println("\n6. Using QuickRender helper...")
	simpleDiagram := `
	pie title Go Project Time Allocation
		"Coding" : 40
		"Testing" : 25
		"Documentation" : 15
		"Debugging" : 10
		"Meetings" : 10
	`
	
	pieData, err := mermaid.QuickRender(simpleDiagram, mermaid.FormatSVG)
	if err != nil {
		log.Fatalf("Failed to quick render: %v", err)
	}
	
	err = ioutil.WriteFile("go_pie.svg", pieData, 0644)
	if err != nil {
		log.Fatalf("Failed to write pie chart: %v", err)
	}
	fmt.Println("   ✓ Saved to go_pie.svg")
	
	// Summary
	fmt.Println("\n✅ All examples completed successfully!")
	fmt.Println("\nGenerated files:")
	
	files := []string{
		"go_flowchart.svg",
		"go_sequence.png",
		"go_architecture.png",
		"go_state.jpg",
		"go_pipeline.webp",
		"go_pie.svg",
	}
	
	for _, file := range files {
		if info, err := os.Stat(file); err == nil {
			fmt.Printf("  - %s (%d bytes)\n", file, info.Size())
		}
	}
}

// Helper function to format bytes
func formatBytes(bytes int64) string {
	const unit = 1024
	if bytes < unit {
		return fmt.Sprintf("%d B", bytes)
	}
	div, exp := int64(unit), 0
	for n := bytes / unit; n >= unit; n /= unit {
		div *= unit
		exp++
	}
	return fmt.Sprintf("%.1f %cB", float64(bytes)/float64(div), "KMGTPE"[exp])
}