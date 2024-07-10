use crate::grid::Grid;
use crate::grid::candidate::Candidate;
use crate::grid::cell::CellSet;

use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_houses().iter()
        .flat_map(|house| grid.values_missing_from(house).into_iter().map(move |value| (house, value)))
        .flat_map(|(house, value)| find_for_house_and_value(grid, house, value))
}

pub fn deductions<const N: usize>(_grid: &Grid<N>, pointing_claiming: &Step<N>) -> Vec<Deduction<N>> {
    match pointing_claiming {
        Step::PointingClaiming { neighbours, value, .. } =>
            neighbours.iter().map(|cell| Deduction::Elimination(cell, *value)).collect(),
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, pointing_claiming: &Step<N>) -> String {
    match pointing_claiming {
        Step::PointingClaiming { house, value, .. } => format!(
            "Pointing/Claiming; the {}s in {} eliminate further {}s from common neighbours",
            value.0, grid.cell_set_name(house), value.0
        ),
        _ => unreachable!(),
    }
}

fn find_for_house_and_value<const N: usize>(grid: &Grid<N>, house: &CellSet<N>, value: Candidate<N>) -> Option<Step<N>> {
    let cells = grid.cells_with_candidate_in(house, value);
    let common_neighbours = grid.common_neighbours(&cells);
    let elimination_cells = grid.cells_with_candidate_in(&common_neighbours, value);
    
    if !elimination_cells.is_empty() {
        Some(Step::PointingClaiming { house: house.clone(), neighbours: elimination_cells, value })
    } else {
        None
    }
}
