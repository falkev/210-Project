mod tourism_graph;
mod gdp_and_tourism__graph;
mod unemployment_graph;
mod cost_index_graph;

use tourism_graph::{read_tourism_csv, visualize_graph};
use gdp_and_tourism__graph::{read_tourism_gdp_csv, visualize_tourism_gdp};
use unemployment_graph::{read_unemployment_csv, visualize_unemployment_graph};
use cost_index_graph::{read_cost_index_csv, visualize_cost_index_graph};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tourism_file_path = "tourism.csv";
    let gdp_file_path = "richest_countries.csv";
    let unemployment_file_path = "unemployment.csv";
    let cost_index_file_path = "cost_of_living.csv";

    let tourism_threshold = 50.0;
    let gdp_threshold = 0.05;
    let unemployment_threshold = 0.05;
    let cost_index_threshold = 0.05;

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

    let gdp_data = match read_tourism_gdp_csv(gdp_file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read GDP data from {}: {}", gdp_file_path, e);
            return Err(e.into());
        }
    };

    if let Err(e) = visualize_tourism_gdp(tourism_data.clone(), gdp_data, gdp_threshold, "tourism_gdp_graph.png") {
        eprintln!("Failed to visualize Tourism-to-GDP graph: {}", e);
        return Err(e.into());
    }
    println!("Tourism-to-GDP graph saved to tourism_gdp_graph.png");

    let unemployment_data = match read_unemployment_csv(unemployment_file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read unemployment data from {}: {}", unemployment_file_path, e);
            return Err(e.into());
        }
    };

    if let Err(e) = visualize_unemployment_graph(tourism_data.clone(), unemployment_data, unemployment_threshold, "tourism_unemployment_graph.png") {
        eprintln!("Failed to visualize Tourism-to-Unemployment graph: {}", e);
        return Err(e.into());
    }
    println!("Tourism-to-Unemployment graph saved to tourism_unemployment_graph.png");

    let cost_index_data = match read_cost_index_csv(cost_index_file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read cost index data from {}: {}", cost_index_file_path, e);
            return Err(e.into());
        }
    };

    if let Err(e) = visualize_cost_index_graph(tourism_data.clone(), cost_index_data, cost_index_threshold, "tourism_cost_index_graph.png") {
        eprintln!("Failed to visualize Tourism-to-Cost Index graph: {}", e);
        return Err(e.into());
    }
    println!("Tourism-to-Cost Index graph saved to tourism_cost_index_graph.png");

    Ok(())
}
