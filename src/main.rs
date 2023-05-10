use csv::Writer;
use rand::prelude::*;

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

fn main() {
    let n = 10;
    let m = 10;
    let steps = 100;
    let size = 1000;
    let stats = simulate(n, m, steps, size);

    let mut wtr = Writer::from_path("stats.csv").expect("Unable to create csv");
    wtr.write_record(&["p", "θ(p)"])
        .expect("Unable to write record");
    for (p, theta) in stats {
        wtr.serialize((p, theta)).expect("Unable to write record");
    }
    wtr.flush().expect("Unable to flush");

    println!("Done!");
}
