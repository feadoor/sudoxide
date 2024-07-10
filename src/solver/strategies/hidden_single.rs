use crate::grid::Grid;
use crate::grid::candidate::Candidate;
use crate::grid::cell::CellSet;

use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_houses().iter()
        .flat_map(|house| grid.values_missing_from(house).into_iter().map(move |value| (house, value)))
        .flat_map(|(house, value)| find_for_house_and_value(grid, house, value))
}

pub fn deductions<const N: usize>(_grid: &Grid<N>, hidden_single: &Step<N>) -> Vec<Deduction<N>> {
    match hidden_single {
        Step::HiddenSingle { cell, value, .. } => vec![Deduction::Placement(*cell, *value)],
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, hidden_single: &Step<N>) -> String {
    match hidden_single {
        Step::HiddenSingle { house, cell, value } => format!(
            "Hidden Single; {} is the only place for {} in {}",
            grid.cell_name(*cell), value.0, grid.cell_set_name(house)
        ),
        _ => unreachable!(),
    }
}

fn find_for_house_and_value<const N: usize>(grid: &Grid<N>, house: &CellSet<N>, value: Candidate<N>) -> Option<Step<N>> {
    let cells = grid.cells_with_candidate_in(house, value);
    if cells.len() == 0 {
        Some(Step::NoPlaceForCandidateInHouse { house: house.clone(), value })
    } else if cells.len() == 1 {
        let cell = cells.first().unwrap();
        Some(Step::HiddenSingle { house: house.clone(), cell, value })
    } else {
        None
    }
}
