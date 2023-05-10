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

fn main() {
    let n = 10;
    let m = 10;
    let p = 0.5;
    let matrix = create_matrix(n, m, p);
    for i in 0..n {
        for j in 0..m {
            print!("{}", matrix[i][j]);
        }
        println!();
    }
    println!("Has path: {}", has_path(&matrix));
}
