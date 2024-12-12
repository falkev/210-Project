mod graphing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tourism_file = "tourism.csv";
    let unemployment_file = "unemployment.csv";
    let output_file = "scatter_plot.png";

    // Call the function from the graphing module
    graphing::create_scatter_plot(tourism_file, unemployment_file, output_file)?;

    Ok(())
}
