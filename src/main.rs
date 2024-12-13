mod tourism_graph;
mod gdp_and_tourism__graph;
mod unemployment_graph;
mod cost_index_graph;
mod purchasing_power_graph;

use tourism_graph::{read_tourism_csv, visualize_graph};
use gdp_and_tourism__graph::{read_tourism_gdp_csv, visualize_tourism_gdp};
use unemployment_graph::{read_unemployment_csv, visualize_unemployment_graph};
use cost_index_graph::{read_cost_index_csv, visualize_cost_index_graph};
use purchasing_power_graph::{read_purchasing_power_csv, visualize_tourism_purchasing_power};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tourism_file_path = "tourism.csv";
    let gdp_file_path = "richest_countries.csv";
    let unemployment_file_path = "unemployment.csv";
    let cost_index_file_path = "cost_of_living.csv";
    let tourism_threshold = 50.0;
    let gdp_threshold = 0.05;
    let cost_index_threshold = 0.05;
    let purchasing_power_threshold = 0.05;

    let tourism_data = read_tourism_csv(tourism_file_path)?;
    visualize_graph(&tourism_data, tourism_threshold, "refined_tourism_graph.png")?;
    println!("Tourism graph saved to refined_tourism_graph.png");

    let gdp_data = read_tourism_gdp_csv(gdp_file_path)?;
    visualize_tourism_gdp(tourism_data.clone(), gdp_data, gdp_threshold, "tourism_gdp_graph.png")?;
    println!("Tourism-to-GDP graph saved to tourism_gdp_graph.png");

    let unemployment_data = read_unemployment_csv(unemployment_file_path)?;
    visualize_unemployment_graph(
        tourism_data.clone(),
        unemployment_data,
        gdp_threshold,
        "tourism_unemployment_graph.png",
    )?;
    println!("Tourism-to-Unemployment graph saved to tourism_unemployment_graph.png");

    let cost_index_data = read_cost_index_csv(cost_index_file_path)?;
    visualize_cost_index_graph(
        tourism_data.clone(),
        cost_index_data,
        cost_index_threshold,
        "tourism_cost_index_graph.png",
    )?;
    println!("Tourism-to-Cost Index graph saved to tourism_cost_index_graph.png");

    let purchasing_power_data = read_purchasing_power_csv(cost_index_file_path)?;
    visualize_tourism_purchasing_power(
        tourism_data,
        purchasing_power_data,
        purchasing_power_threshold,
        "tourism_purchasing_power_graph.png",
    )?;
    println!("Tourism-to-Purchasing Power graph saved to tourism_purchasing_power_graph.png");

    Ok(())
}
