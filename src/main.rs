mod tourism_graph; 
mod gdp_and_tourism__graph; 

use tourism_graph::{read_tourism_csv, visualize_graph};
use gdp_and_tourism__graph::{read_tourism_gdp_csv, visualize_tourism_gdp};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tourism_file_path = "tourism.csv"; 
    let gdp_file_path = "richest_countries.csv";
    let tourism_threshold = 50.0;
    let gdp_threshold = 0.05;

    //Generate the Tourism Graph
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

    //Generate the Tourism-to-GDP Graph
    let gdp_data = match read_tourism_gdp_csv(gdp_file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read GDP data from {}: {}", gdp_file_path, e);
            return Err(e.into());
        }
    };

    if let Err(e) = visualize_tourism_gdp(tourism_data, gdp_data, gdp_threshold, "tourism_gdp_graph.png") {
        eprintln!("Failed to visualize Tourism-to-GDP graph: {}", e);
        return Err(e.into());
    }
    println!("Tourism-to-GDP graph saved to tourism_gdp_graph.png");

    Ok(())
}
