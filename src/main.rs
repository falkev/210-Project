mod tourism_graph;
mod gdp_and_tourism_graph;
mod unemployment_graph;

use tourism_graph::{read_tourism_csv, visualize_graph};
use gdp_and_tourism_graph::{read_tourism_gdp_csv, visualize_tourism_gdp};
use unemployment_graph::{read_unemployment_csv, visualize_unemployment_tourism};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tourism_file_path = "tourism.csv";
    let gdp_file_path = "richest_countries.csv";
    let unemployment_file_path = "unemployment.csv";

    let tourism_threshold = 50.0;
    let gdp_threshold = 0.05;
    let unemployment_threshold = 0.1;

    // Generate the Tourism Graph
    let tourism_data = match read_tourism_csv(tourism_file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read tourism data from {}: {}", tourism_file_path, e);
            return Err(e.into());
        }
    };

    if let Err(e) = visualize_graph(&tourism_data, tourism_threshold, "refined_tourism_graph.png") {
        eprintln!("Failed to visualize tourism graph: {}", e);
        return Err(e.into());
    }
    println!("Tourism graph saved to refined_tourism_graph.png");

    // Generate the Tourism-to-GDP Graph
    let gdp_data = match read_tourism_gdp_csv(gdp_file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read GDP data from {}: {}", gdp_file_path, e);
            return Err(e.into());
        }
    };

    if let Err(e) =
        visualize_tourism_gdp(tourism_data.clone(), gdp_data, gdp_threshold, "tourism_gdp_graph.png")
    {
        eprintln!("Failed to visualize Tourism-to-GDP graph: {}", e);
        return Err(e.into());
    }
    println!("Tourism-to-GDP graph saved to tourism_gdp_graph.png");

    // Generate the Tourism-to-Unemployment Graph
    let unemployment_data = match read_unemployment_csv(unemployment_file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!(
                "Failed to read unemployment data from {}: {}",
                unemployment_file_path, e
            );
            return Err(e.into());
        }
    };

    if let Err(e) = visualize_unemployment_tourism(
        tourism_data,
        unemployment_data,
        unemployment_threshold,
        "tourism_unemployment_graph.png",
    ) {
        eprintln!("Failed to visualize Tourism-to-Unemployment graph: {}", e);
        return Err(e.into());
    }
    println!("Tourism-to-Unemployment graph saved to tourism_unemployment_graph.png");

    Ok(())
}
