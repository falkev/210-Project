use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use plotters::prelude::*;



pub fn read_tourism_gdp_csv(file_path: &str) -> Result<Vec<(String, f64)>, Box<dyn Error>> {
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

        data.push((country, gdp_per_capita));
    }

    Ok(data)
}

pub fn visualize_tourism_gdp(
    tourism_data: Vec<(String, f64)>,
    gdp_data: Vec<(String, f64)>,
    threshold: f64,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = petgraph::graph::UnGraph::<String, ()>::new_undirected();
    let mut nodes: Vec<(String, f64)> = vec![];

    // Merge tourism and GDP data
    let gdp_map: HashMap<_, _> = gdp_data.into_iter().collect();
    let combined_data: Vec<(String, f64)> = tourism_data
        .into_iter()
        .filter_map(|(country, tourism)| {
            gdp_map.get(&country).map(|&gdp| (country, tourism / gdp))
        })
        .collect();

    // Generate positions for nodes using a circular layout
    let positions: Vec<(i32, i32)> = (0..combined_data.len())
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (combined_data.len() as f64);
            let x = (angle.cos() * 200.0) as i32;
            let y = (angle.sin() * 200.0) as i32;
            (x, y)
        })
        .collect();

    // Prepare Plotters backend
    let root = BitMapBackend::new(output_file, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Tourism-to-GDP-per-capita Graph", ("sans-serif", 30))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(-250..250, -250..250)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Draw nodes
    for (pos, (country, ratio)) in positions.iter().zip(combined_data.iter()) {
        let size = (ratio * 10.0) as i32;
        chart.draw_series(std::iter::once(Circle::new(
            *pos,
            size.clamp(5, 15),
            RED.filled(),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            format!("{} ({:.2})", country, ratio),
            *pos,
            ("sans-serif", 12),
        )))?;
    }

    // Draw edges
    for i in 0..combined_data.len() {
        for j in (i + 1)..combined_data.len() {
            if (combined_data[i].1 - combined_data[j].1).abs() <= threshold {
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![positions[i], positions[j]],
                    &BLACK,
                )))?;
            }
        }
    }

    root.present()?; 

    Ok(())
}
