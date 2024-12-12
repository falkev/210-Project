mod graphing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "tourism.csv"; // Path to your CSV file
    let threshold = 50.0; // Set the threshold value

    // Read data from the CSV file
    let tourism_data = graphing::read_tourism_csv(file_path)?;

    // Refined graph with labels and scaling
    graphing::visualize_graph(tourism_data.clone(), threshold, "refined_tourism_graph.png")?;

    println!("Graph saved to refined_tourism_graph.png");

    Ok(())
}
