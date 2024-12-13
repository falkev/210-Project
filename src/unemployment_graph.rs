use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use plotters::prelude::*;

pub fn read_unemployment_csv(file_path: &str) -> Result<Vec<(String, f64)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index == 0 {
            continue; // Skip header
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid data format at line {}", index + 1).into());
        }

        let country = parts[0].to_string();
        let unemployment_rate: f64 = parts[1].trim().parse()?;

        data.push((country, unemployment_rate));
    }

    Ok(data)
}

pub fn visualize_unemployment_tourism(
    tourism_data: Vec<(String, f64)>,
    unemployment_data: Vec<(String, f64)>,
    threshold: f64,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let unemployment_map: HashMap<_, _> = unemployment_data.into_iter().collect();
    let combined_data: Vec<(String, f64)> = tourism_data
        .into_iter()
        .filter_map(|(country, tourism)| {
            unemployment_map.get(&country).and_then(|&unemployment| {
                if unemployment > 0.0 && tourism > 0.0 {
                    Some((country, tourism / unemployment))
                } else {
                    None
                }
            })
        })
        .collect();

    // Normalize the ratios for better scaling
    let min_ratio = combined_data
        .iter()
        .map(|(_, r)| *r)
        .fold(f64::INFINITY, f64::min);
    let max_ratio = combined_data
        .iter()
        .map(|(_, r)| *r)
        .fold(f64::NEG_INFINITY, f64::max);

    let scaled_data: Vec<(String, f64)> = combined_data
        .iter()
        .map(|(country, ratio)| (country.clone(), (ratio - min_ratio) / (max_ratio - min_ratio)))
        .collect();

    // Generate positions for nodes using a circular layout
    let positions: Vec<(i32, i32)> = (0..scaled_data.len())
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (scaled_data.len() as f64);
            let x = (angle.cos() * 200.0) as i32;
            let y = (angle.sin() * 200.0) as i32;
            (x, y)
        })
        .collect();

    // Prepare Plotters backend
    let root = BitMapBackend::new(output_file, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Tourism-to-Unemployment Graph", ("sans-serif", 30))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(-250..250, -250..250)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Draw nodes
    for (pos, (country, scaled_ratio)) in positions.iter().zip(scaled_data.iter()) {
        let size = ((scaled_ratio * 20.0) as i32).clamp(5, 20); // Adjusted size range
        chart.draw_series(std::iter::once(Circle::new(
            *pos,
            size,
            BLUE.filled(),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            format!("{} ({:.2})", country, scaled_ratio),
            *pos,
            ("sans-serif", 12),
        )))?;
    }

    // Draw edges
    for i in 0..scaled_data.len() {
        for j in (i + 1)..scaled_data.len() {
            if (scaled_data[i].1 - scaled_data[j].1).abs() <= threshold {
                let edge_weight = 1.0 - (scaled_data[i].1 - scaled_data[j].1).abs();
                let edge_color = BLACK.mix(edge_weight as f64);
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![positions[i], positions[j]],
                    ShapeStyle {
                        color: edge_color,
                        filled: false,
                        stroke_width: (2.0 * edge_weight) as u32,
                    },
                )))?;
            }
        }
    }

    // Add legend
    chart.draw_series(std::iter::once(Text::new(
        "Node size: Tourism-to-Unemployment ratio\nEdge thickness: Similarity",
        (-200, 220),
        ("sans-serif", 12),
    )))?;

    root.present()?;

    Ok(())
}
