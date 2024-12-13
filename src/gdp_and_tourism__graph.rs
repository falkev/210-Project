
use std::error::Error;
use plotters::prelude::*;

/// Read tourism and GDP per capita data from a CSV file and calculate the Tourism-to-GDP-per-capita ratio
pub fn read_tourism_gdp_csv(file_path: &str) -> Result<Vec<(String, f64)>, Box<dyn Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index == 0 {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid data format at line {}", index + 1).into());
        }

        let country = parts[0].to_string();
        let gdp_per_capita: f64 = parts[1].trim().parse()?;

        // Store country and GDP per capita in the data vector
        data.push((country, gdp_per_capita));
    }

    Ok(data)
}

/// Visualize the Tourism-to-GDP-per-capita ratio as a graph
pub fn visualize_tourism_gdp(
    tourism_data: Vec<(String, f64)>,
    gdp_data: Vec<(String, f64)>,
    threshold: f64,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let mut graph = petgraph::graph::UnGraph::<String, ()>::new_undirected();
    let mut nodes = vec![];

    // Merge tourism data and GDP data by country
    let gdp_map: std::collections::HashMap<_, _> = gdp_data.into_iter().collect();
    let combined_data: Vec<(String, f64)> = tourism_data
        .into_iter()
        .filter_map(|(country, tourism)| {
            gdp_map.get(&country).map(|&gdp| (country, tourism / gdp))
        })
        .collect();

    // Add nodes
    for (country, _) in &combined_data {
        let node_index = graph.add_node(country.clone());
        nodes.push((country.clone(), node_index));
    }

    // Add edges based on threshold
    for i in 0..combined_data.len() {
        for j in (i + 1)..combined_data.len() {
            if (combined_data[i].1 - combined_data[j].1).abs() <= threshold {
                graph.add_edge(nodes[i].1, nodes[j].1, ());
            }
        }
    }

    // Prepare positions for visualization
    let positions: Vec<(i32, i32)> = (0..combined_data.len())
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (combined_data.len() as f64);
            let x = (angle.cos() * 90.0) as i32; // Scale to range -100 to 100
            let y = (angle.sin() * 90.0) as i32;
            (x, y)
        })
        .collect();

    // Visualize using Plotters
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Tourism-to-GDP-per-capita Graph Visualization", ("sans-serif", 30))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(-100..100, -100..100)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Plot nodes as circles with labels
    for (pos, (country, ratio)) in positions.iter().zip(combined_data.iter()) {
        let size = (ratio * 10.0) as i32; // Adjust size based on ratio
        chart.draw_series(std::iter::once(Circle::new(
            *pos,
            size.clamp(3, 10), // Clamp size to a reasonable range
            RED.filled(),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            format!("{} ({:.2})", country, ratio),
            *pos,
            ("sans-serif", 15),
        )))?;
    }

    // Plot edges with varying thickness
    for i in 0..combined_data.len() {
        for j in (i + 1)..combined_data.len() {
            if (combined_data[i].1 - combined_data[j].1).abs() <= threshold {
                let weight = 1 + (5 - (combined_data[i].1 - combined_data[j].1).abs() as i32).clamp(1, 5); // Edge thickness
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![positions[i], positions[j]],
                    &BLACK.mix(0.2 * weight as f64), // Adjust line thickness
                )))?;
            }
        }
    }

    Ok(())
}

