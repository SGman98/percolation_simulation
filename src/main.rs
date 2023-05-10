use clap::Parser;
use csv::Writer;
use plotters::prelude::*;
use rand::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    #[arg(help = "Number of rows in the matrix", short, default_value_t = 10)]
    n: usize,
    #[arg(help = "Number of columns in the matrix", short, default_value_t = 10)]
    m: usize,
    #[arg(help = "Number of steps in the simulation", long, default_value_t = 100)]
    steps: usize,
    #[arg(
        help = "Number of simulations to run for each step",
        long,
        default_value_t = 1000
    )]
    size: usize,
}

fn create_matrix(n: usize, m: usize, p: f64) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut matrix = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            matrix[i][j] = if rng.gen::<f64>() < p { 1 } else { 0 };
        }
    }
    matrix
}

// Helper function for DFS traversal
fn dfs(matrix: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    let n = matrix.len();
    let m = matrix[0].len();
    if i == n - 1 {
        return true;
    }
    visited[i][j] = true;
    let mut res = false;
    if i > 0 && !visited[i - 1][j] && matrix[i - 1][j] == 1 {
        res |= dfs(matrix, visited, i - 1, j);
    }
    if i < n - 1 && !visited[i + 1][j] && matrix[i + 1][j] == 1 {
        res |= dfs(matrix, visited, i + 1, j);
    }
    if j > 0 && !visited[i][j - 1] && matrix[i][j - 1] == 1 {
        res |= dfs(matrix, visited, i, j - 1);
    }
    if j < m - 1 && !visited[i][j + 1] && matrix[i][j + 1] == 1 {
        res |= dfs(matrix, visited, i, j + 1);
    }
    res
}

fn has_path(matrix: &Vec<Vec<u8>>) -> bool {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut visited = vec![vec![false; m]; n];
    for j in 0..m {
        if dfs(matrix, &mut visited, 0, j) {
            return true;
        }
    }
    false
}

fn simulate(n: usize, m: usize, steps: usize, size: usize) -> Vec<(f64, f64)> {
    let mut stats = Vec::new();
    let step_size = 1.0 / steps as f64;
    for i in 0..steps {
        let p = step_size * i as f64;
        let mut num_paths = 0;
        for _ in 0..size {
            let matrix = create_matrix(n, m, p);
            if has_path(&matrix) {
                num_paths += 1;
            }
        }
        let theta = num_paths as f64 / size as f64;
        stats.push((p, theta));
    }
    stats
}

fn plot_stats(stats: &[(f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = 0.0;
    let x_max = 1.0;
    let y_min = 0.0;
    let y_max = 1.0;

    let mut chart = ChartBuilder::on(&root)
        .caption("Percolation Threshold", ("sans-serif", 50))
        .margin(5)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().x_labels(10).y_labels(10).draw()?;

    chart.draw_series(LineSeries::new(
        stats.iter().map(|(p, theta)| (*p, *theta)),
        &RED,
    ))?;

    Ok(())
}

fn main() {
    let config = Config::parse();
    let stats = simulate(config.n, config.m, config.steps, config.size);

    plot_stats(&stats).expect("Unable to plot");

    let mut wtr = Writer::from_path("stats.csv").expect("Unable to create csv");
    wtr.write_record(&["p", "θ(p)"])
        .expect("Unable to write record");
    for (p, theta) in stats {
        wtr.serialize((p, theta)).expect("Unable to write record");
    }
    wtr.flush().expect("Unable to flush");

    println!("Done!");
}
