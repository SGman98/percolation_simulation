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
}
