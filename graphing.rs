use plotters::prelude::*;
use csv::ReaderBuilder;
use std::collections::HashMap;

pub fn create_scatter_plot(
    tourism_file: &str,
    unemployment_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tourism_data = ReaderBuilder::new()
        .has_headers(true)
        .from_path(tourism_file)?;
    let mut tourism_map = HashMap::new();

    for record in tourism_data.records() {
        let record = record?;
        let country = record.get(0).unwrap().to_string();
        let tourism_revenue: f64 = record.get(1).unwrap().parse().unwrap();
        tourism_map.insert(country, tourism_revenue);
    }

    let mut unemployment_data = ReaderBuilder::new()
        .has_headers(true)
        .from_path(unemployment_file)?;
    let mut data_points = vec![];

    for record in unemployment_data.records() {
        let record = record?;
        let country = record.get(0).unwrap();
        let unemployment_rate: f64 = record.get(1).unwrap().parse().unwrap();

        if let Some(&tourism_revenue) = tourism_map.get(country) {
            data_points.push((tourism_revenue, unemployment_rate));
        }
    }

    let root_area = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Tourism Revenue vs Unemployment Rate", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..120f64, 0f64..50f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        data_points
            .iter()
            .map(|&(x, y)| Circle::new((x, y), 5, BLUE.filled())),
    )?;

    println!("Scatter plot saved to {}", output_file);
    Ok(())
}
