use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::graph::UnGraph;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Represents a graph with adjacency list
pub struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    /// Create a new, empty graph
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, node1: String, node2: String) {
        self.adjacency_list
            .entry(node1.clone())
            .or_insert_with(Vec::new)
            .push(node2.clone());
        self.adjacency_list
            .entry(node2)
            .or_insert_with(Vec::new)
            .push(node1);
    }

    /// Compute the average shortest path distance using BFS
    pub fn average_shortest_path(&self) -> f64 {
        let mut total_distance = 0;
        let mut pair_count = 0;

        for start in self.adjacency_list.keys() {
            let distances = self.bfs_distances(start);
            for (_, distance) in distances {
                total_distance += distance;
                pair_count += 1;
            }
        }

        if pair_count > 0 {
            total_distance as f64 / pair_count as f64
        } else {
            0.0
        }
    }

    /// Perform BFS to calculate distances from a starting node
    fn bfs_distances(&self, start: &String) -> HashMap<String, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start.clone(), 0));
        visited.insert(start.clone());

        while let Some((current, distance)) = queue.pop_front() {
            distances.insert(current.clone(), distance);

            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back((neighbor.clone(), distance + 1));
                    }
                }
            }
        }

        distances
    }
}

/// Build a graph based on tourism data
pub fn build_tourism_graph(data: Vec<(String, f64)>, threshold: f64) -> Graph {
    let mut graph = Graph::new();

    for (i, (country1, tourism1)) in data.iter().enumerate() {
        for (country2, tourism2) in data.iter().skip(i + 1) {
            if (tourism1 - tourism2).abs() <= threshold {
                graph.add_edge(country1.clone(), country2.clone());
            }
        }
    }

    graph
}

pub fn visualize_graph(
    data: Vec<(String, f64)>,
    threshold: f64,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut nodes = vec![];

    // Add nodes
    for (country, _) in &data {
        let node_index = graph.add_node(country.clone());
        nodes.push((country.clone(), node_index));
    }

    // Add edges based on threshold
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if (data[i].1 - data[j].1).abs() <= threshold {
                graph.add_edge(nodes[i].1, nodes[j].1, ());
            }
        }
    }

    // Prepare positions for visualization
    let positions: Vec<(i32, i32)> = (0..data.len())
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (data.len() as f64);
            let x = (angle.cos() * 90.0) as i32; // Scale to range -100 to 100
            let y = (angle.sin() * 90.0) as i32;
            (x, y)
        })
        .collect();

    // Visualize using Plotters
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Tourism Graph Visualization", ("sans-serif", 30))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(-100..100, -100..100)?;

    chart.configure_mesh().disable_mesh().draw()?;

    // Plot nodes as circles with varying sizes and labels
    for (pos, (country, revenue)) in positions.iter().zip(data.iter()) {
        let size = (revenue / 100.0) as i32; // Adjust size based on revenue
        chart.draw_series(std::iter::once(Circle::new(
            *pos,
            size.clamp(3, 10), // Clamp size to a reasonable range
            BLUE.filled(),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            country.clone(), // Clone the country name for use as a label
            *pos,
            ("sans-serif", 15),
        )))?;
    }

    // Plot edges with varying thickness
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if (data[i].1 - data[j].1).abs() <= threshold {
                let weight = 1 + (5 - (data[i].1 - data[j].1).abs() as i32).clamp(1, 5); // Edge thickness
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![positions[i], positions[j]],
                    &BLACK.mix(0.2 * weight as f64), // Adjust line thickness
                )))?;
            }
        }
    }

    Ok(())
}

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
        let revenue: f64 = parts[1].trim().parse()?;
        data.push((country, revenue));
    }

    Ok(data)
}
