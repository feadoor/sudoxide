use patterns::PatternPuzzlesIterator;

use crate::grid::Grid;
use crate::grid::cell::CellIdx;

mod brute_force;
mod canonicalisation;
mod patterns;

pub fn generate_puzzles_on_empty_grid_with_pattern<const N: usize>(grid: Grid<N>, pattern: Vec<CellIdx<N>>) -> impl Iterator<Item = Vec<usize>> {
    PatternPuzzlesIterator::for_empty_grid_and_pattern(grid, pattern)
}

pub fn generate_puzzles_for_starting_grid_with_pattern<const N: usize>(grid: Grid<N>, pattern: Vec<CellIdx<N>>) -> impl Iterator<Item = Vec<usize>> {
    PatternPuzzlesIterator::for_starting_grid_and_pattern(grid, pattern)
}
