use itertools::Itertools;

use crate::grid::Grid;
use crate::grid::candidate::CandidateSet;
use crate::grid::cell::CellSet;

use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>, degree: usize) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_houses().iter()
        .filter(move |house| grid.empty_cells_in(house).len() > 2 * degree)
        .flat_map(move |house| grid.values_missing_from(house).into_iter().combinations(degree).map(CandidateSet::from_candidates).map(move |values| (house, values)))
        .flat_map(move |(house, values)| find_for_house_and_values(grid, degree, house, &values))
}

pub fn deductions<const N: usize>(grid: &Grid<N>, hidden_subset: &Step<N>) -> Vec<Deduction<N>> {
    match hidden_subset {
        Step::HiddenSubset { cells, values, .. } => _deductions(grid, cells, values),
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, hidden_subset: &Step<N>) -> String {
    match hidden_subset {
        Step::HiddenSubset { house, cells, values } => format!(
            "Hidden {}; {} in {} {}",
            subset_name(cells.len()), values, grid.cell_set_name(house), grid.cell_set_name(cells),
        ),
        _ => unreachable!(),
    }
}

fn find_for_house_and_values<const N: usize>(grid: &Grid<N>, degree: usize, house: &CellSet<N>, values: &CandidateSet<N>) -> Option<Step<N>> {
    let cells = grid.cells_with_any_of_candidates_in(house, &values);
    if cells.len() == degree && cells.iter().any(|cell| grid.candidates(cell).intersects(&!values)) {
        Some(Step::HiddenSubset { house: house.clone(), cells, values: values.clone() })
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
    let other_values = !values;
    cells.iter().flat_map(|cell| {
        (grid.candidates(cell) & &other_values).into_iter().map(move |value| Deduction::Elimination(cell, value))
    }).collect()
}
