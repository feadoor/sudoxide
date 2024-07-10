use std::io::{self, BufRead};

use itertools::Itertools;

use sudoxide::grid::Grid;
use sudoxide::grid::variants::Classic;
use sudoxide::solver;
use sudoxide::solver::strategies::all_strategies;

const N: usize = 9;

fn main() {

    let stdin = io::stdin();

    println!("Enter a sudoku:");

    for line in stdin.lock().lines() {
        let grid_result = Grid::<N>::from_empty_grid_and_string(Grid::empty_classic(), &line.expect("Failed to read from stdin"));
        if grid_result.is_ok() {
            let mut grid = grid_result.unwrap();
            println!("\nInitial grid:\n\n{}", grid);
            let solve_details = solver::solve(&mut grid, &all_strategies(N));
            for (step, deductions) in solve_details.steps {
                println!("- {} ({})", step.description(&grid), deductions.iter().map(|d| d.description(&grid)).join(", "));
            }
            println!("\nResult: {:?}", solve_details.result);
            println!("\nFinal grid:\n\n{}", grid);
        }
        else {
            println!("{}", grid_result.err().unwrap())
        }

        println!("\nEnter a sudoku:");
    }
}
