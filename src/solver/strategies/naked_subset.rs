use itertools::Itertools;

use crate::grid::{candidate::CandidateSet, Grid};
use crate::grid::cell::CellSet;

use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>, degree: usize) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_houses().iter()
        .filter(move |house| grid.empty_cells_in(house).len() >= 2 * degree)
        .flat_map(move |house| grid.empty_cells_in(house).into_iter().combinations(degree).map(CellSet::from_cells))
        .flat_map(move |cells| find_for_cells(grid, degree, &cells))
}

pub fn deductions<const N: usize>(grid: &Grid<N>, naked_subset: &Step<N>) -> Vec<Deduction<N>> {
    match naked_subset {
        Step::NakedSubset { cells, values } => _deductions(grid, cells, values),
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, naked_subset: &Step<N>) -> String {
    match naked_subset {
        Step::NakedSubset { cells, values } => format!(
            "Naked {}; {} in {}",
            subset_name(cells.len()), values, grid.cell_set_name(cells)
        ),
        _ => unreachable!(),
    }
}

fn find_for_cells<const N: usize>(grid: &Grid<N>, degree: usize, cells: &CellSet<N>) -> Option<Step<N>> {
    let candidates = grid.candidates_in(cells);
    if candidates.len() == degree && grid.common_neighbours(cells).iter().any(|cell| grid.candidates(cell).intersects(&candidates)) {
        Some(Step::NakedSubset { cells: cells.clone(), values: candidates })
    } else {
        None
    }
}

fn subset_name<'a>(size: usize) -> &'a str {
    match size {
        2 => "Pair", 3 => "Triple", 4 => "Quad", 
        5 => "Quint", 6 => "Sextuple", 7 => "Septuple", 
        8 => "Octuple", 9 => "Nonuple", 10 => "Decuple",
        _ => "Subset"
    }
}

fn _deductions<const N: usize>(grid: &Grid<N>, cells: &CellSet<N>, values: &CandidateSet<N>) -> Vec<Deduction<N>> {
    grid.common_neighbours(cells).iter().flat_map(|cell| {
        (grid.candidates(cell) & values).into_iter().map(move |value| Deduction::Elimination(cell, value))
    }).collect()
}
