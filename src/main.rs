mod tourism_graph; // Matches the file name `src/tourism_graph.rs`
mod gdp_and_tourism_graph; // Matches the file name `src/gdp_and_tourism_graph.rs`

use tourism_graph::{read_tourism_csv, visualize_graph};
use gdp_and_tourism_graph::{read_tourism_gdp_csv, visualize_tourism_gdp};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tourism_file_path = "tourism.csv"; // Path to your Tourism CSV file
    let gdp_file_path = "richest_countries.csv"; // Path to your GDP CSV file
    let tourism_threshold = 50.0; // Threshold for the tourism graph
    let gdp_threshold = 0.05; // Threshold for the Tourism-to-GDP graph

    // Step 1: Generate the Tourism Graph
    let tourism_data = read_tourism_csv(tourism_file_path)?;
    visualize_graph(tourism_data.clone(), tourism_threshold, "refined_tourism_graph.png")?;
    println!("Tourism graph saved to refined_tourism_graph.png");

    // Step 2: Generate the Tourism-to-GDP Graph
    let gdp_data = read_tourism_gdp_csv(gdp_file_path)?;
    visualize_tourism_gdp(
        tourism_data,
        gdp_data,
        gdp_threshold,
        "tourism_gdp_graph.png",
    )?;
    println!("Tourism-to-GDP-per-capita graph saved to tourism_gdp_graph.png");

    Ok(())
}
