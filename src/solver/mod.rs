use strategies::{Deduction, Step, Strategy};

use crate::grid::Grid;

pub mod strategies;

#[derive(PartialEq, Eq, Debug)]
pub enum SolveResult {
    Solved,
    Contradiction,
    InsufficientStrategies,
}

pub struct SolveDetails<const N: usize> {
    pub result: SolveResult,
    pub steps: Vec<(Step<N>, Vec<Deduction<N>>)>,
}

pub fn solve<const N: usize>(grid: &mut Grid<N>, strategies: &[Strategy]) -> SolveDetails<N> {

    let mut steps = Vec::new();

    while !grid.is_solved() {
        if let Some((step, deductions)) = find_step(grid, strategies) {
            steps.push((step, deductions.clone()));
            for &deduction in &deductions {
                if let Deduction::Contradiction = deduction {
                    return SolveDetails { result: SolveResult::Contradiction, steps };
                } else {
                    grid.apply_deduction(deduction);
                }
            }
        } else {
            return SolveDetails { result: SolveResult::InsufficientStrategies, steps };
        }
    }

    SolveDetails { result: SolveResult::Solved, steps }
}

fn find_step<const N: usize>(grid: &Grid<N>, strategies: &[Strategy]) -> Option<(Step<N>, Vec<Deduction<N>>)> {
    
    for &strategy in strategies {
        for step in strategy.find_steps(&grid) {
            let deductions = step.deductions(grid);
            if deductions.len() > 0 { return Some((step, deductions)); }
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    use crate::grid::Grid;
    use crate::grid::variants::Classic;
    use crate::solver::strategies::all_strategies;
    use crate::solver::{solve, SolveResult};

    const N: usize = 9;

    fn check_grid<const N: usize>(grid: &Grid<N>) {
        for house in grid.all_houses() {
            for value in grid.all_values().iter() {
                assert!(house.iter().any(|cell| grid.value(cell) == Some(value)))
            }
        }
    }

    #[test]
    fn test_classic_solves() {
        let file = File::open("classic_grids.txt").expect("Input file not present");
        let lines = BufReader::new(file).lines().map(|l| l.expect("Error reading from file"));
        for line in lines.filter(|l| !l.is_empty() && !l.starts_with("//")) {
            let mut grid = Grid::<N>::from_empty_grid_and_string(Grid::<9>::empty_classic(), &line).expect("Failed to parse grid");
            assert_eq!(solve(&mut grid, &all_strategies(N)).result, SolveResult::Solved);
            check_grid(&grid);
        }
    }
}
