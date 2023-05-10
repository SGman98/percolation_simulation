# Percolation Simulation

This simulation generates and analyzes matrices that represent percolation
systems. A percolation system is a matrix of cells that are either open or
closed. A system percolates if there is a path of open cells from the top row
to the bottom row.

## Usage

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository to your local machine.
3. Open a terminal and navigate to the root of the repository.

## Running the Simulation

To run the simulation, use the following command:

```sh
cargo build --release
```

This will build the simulation in release mode. To run the simulation, use the
following command:

```sh
cargo run --release -- --help
```

This will print the help message for the simulation. The simulation takes the
following arguments:

- `-n <N>`: The number of rows in the matrix. Defaults to 10.
- `-m <M>`: The number of columns in the matrix. Defaults to 10.
- `--steps <STEPS>`: The number of steps in the simulation. Defaults to 100.
- `--size <SIZE>`: The number of simulations to run for each step. Defaults to 1000.
- `-h, --help`: Print the help message.
- `-V, --version`: Print the version.

## Results

The simulation will generate a CSV file called `stats.csv`

- `p`: The probability of a cell being open.
- `θ(p)`: The probability that the system percolates.

The simulation will also generate a plot in the `plot.png` file. This plot
will show the probability of the system percolating as a function of the
probability of a cell being open.
