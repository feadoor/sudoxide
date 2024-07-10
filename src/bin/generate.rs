use itertools::Itertools;

use sudoxide::analyser::steps_to_solve;
use sudoxide::grid::candidate::Candidate;
use sudoxide::grid::cell::CellIdx;
use sudoxide::grid::variants::Classic;
use sudoxide::grid::Grid;
use sudoxide::generator;
use sudoxide::solver::strategies::Strategy::*;

fn main() {
    let pattern = [(0, 3), (1, 2), (1, 4), (2, 1), (2, 3), (2, 5), (3, 0), (3, 2), (3, 4), (3, 6), (4, 1), (4, 3), (4, 5), (4, 7), (5, 2), (5, 4), (5, 6), (5, 8), (6, 3), (6, 5), (6, 7), (7, 4), (7, 6), (8, 5)];
    let pattern_cells: Vec<_> = pattern.into_iter().map(|(r, c)| CellIdx::<9>::from_row_and_col(r, c)).collect();
    let competition_steps = vec![vec![FullHouse, HiddenSingle, NakedSingle], vec![PointingClaiming], vec![NakedSubset(2), HiddenSubset(2)], vec![NakedSubset(3), HiddenSubset(3)], vec![NakedSubset(4), HiddenSubset(4)]];
    let empty_grid = Grid::<9>::empty_classic();

    for puzzle in generator::generate_puzzles_on_empty_grid_with_pattern(empty_grid.clone(), pattern_cells) {
        let clues: Vec<_> = puzzle.iter().map(|&v| if v == 0 { None } else { Some(Candidate(v)) }).collect();
        let grid = Grid::<9>::from_empty_grid_and_clues(empty_grid.clone(), &clues).unwrap();
        if let Some(steps) = steps_to_solve(grid, &competition_steps) {
            println!("{} - {}", steps.into_iter().rev().join(" "), puzzle.into_iter().join(""));
        }
    }
}
