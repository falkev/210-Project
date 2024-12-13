use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::graph::UnGraph;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_tourism_csv(file_path: &str) -> Result<Vec<(String, f64)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index == 0 {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 5 {
            return Err(format!("Invalid data format at line {}", index + 1).into());
        }

        let country = parts[0].to_string();
        let revenue: f64 = parts[1].trim().parse().map_err(|e| {
            format!("Invalid revenue value at line {}: {}", index + 1, e)
        })?;
        data.push((country, revenue));
    }

    Ok(data)
}

pub fn visualize_graph(
    data: &Vec<(String, f64)>, 
    threshold: f64,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let positions: Vec<(i32, i32)> = (0..data.len())
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (data.len() as f64);
            let x = (angle.cos() * 200.0) as i32; 
            let y = (angle.sin() * 200.0) as i32;
            (x, y)
        })
        .collect();

    let root = BitMapBackend::new(output_file, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Tourism Graph Visualization", ("sans-serif", 30))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(-250..250, -250..250)?;

    chart.configure_mesh().disable_mesh().draw()?;

    for (pos, (country, revenue)) in positions.iter().zip(data.iter()) {
        let size = (revenue / 10.0) as i32;
        chart.draw_series(std::iter::once(Circle::new(
            *pos,
            size.clamp(5, 15),
            BLUE.filled(),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            country.clone(),
            *pos,
            ("sans-serif", 12),
        )))?;
    }

    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if (data[i].1 - data[j].1).abs() <= threshold {
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
