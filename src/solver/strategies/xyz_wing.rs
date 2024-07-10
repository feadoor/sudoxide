use crate::grid::Grid;
use crate::grid::cell::CellIdx;

use itertools::Itertools;

use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.cells_with_n_candidates(3).into_iter()
        .flat_map(|pivot| find_pincers(grid, pivot).map(move |(p1, p2)| (pivot, p1, p2)))
        .flat_map(|(pivot, p1, p2)| find_for_pivot_and_pincers(grid, pivot, p1, p2))
}

pub fn deductions<const N: usize>(grid: &Grid<N>, xyz_wing: &Step<N>) -> Vec<Deduction<N>> {
    match xyz_wing {
        Step::XYZWing { pivot, pincer1, pincer2, value } => grid
            .cells_with_candidate_in(&(grid.neighbours(*pivot) & grid.neighbours(*pincer1) & grid.neighbours(*pincer2)), *value)
            .iter().map(|cell| Deduction::Elimination(cell, *value))
            .collect(),
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, xyz_wing: &Step<N>) -> String {
    match xyz_wing {
        Step::XYZWing { pivot, pincer1, pincer2, value } => format!(
            "XYZ-Wing; pivot {} and pincers ({}, {}) eliminate {} from common neighbours",
            grid.cell_name(*pivot), grid.cell_name(*pincer1), grid.cell_name(*pincer2), value.0,
        ),
        _ => unreachable!(),
    }
}

fn find_for_pivot_and_pincers<const N: usize>(grid: &Grid<N>, pivot: CellIdx<N>, pincer1: CellIdx<N>, pincer2: CellIdx<N>) -> Option<Step<N>> {
    let common_candidate = (grid.candidates(pincer1) & grid.candidates(pincer2)).first().unwrap();
    let elimination_cells = grid.neighbours(pivot) & grid.neighbours(pincer1) & grid.neighbours(pincer2);
    if grid.candidate_appears_in(&elimination_cells, common_candidate) {
        Some(Step::XYZWing { pivot, pincer1, pincer2, value: common_candidate })
    } else {
        None
    }
}

fn find_pincers<const N: usize>(grid: &Grid<N>, pivot: CellIdx<N>) -> impl Iterator<Item = (CellIdx<N>, CellIdx<N>)> + '_ {
    grid.cells_with_n_candidates_in(grid.neighbours(pivot), 2)
        .into_iter().tuple_combinations()
        .filter(move |&(pincer1, pincer2)| &(grid.candidates(pincer1) | grid.candidates(pincer2)) == grid.candidates(pivot))
}
