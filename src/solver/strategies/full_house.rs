use crate::grid::Grid;
use crate::grid::candidate::Candidate;
use crate::grid::cell::CellSet;
use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_houses().iter().flat_map(|house| find_for_house(grid, house))
}

pub fn deductions<const N: usize>(_grid: &Grid<N>, full_house: &Step<N>) -> Vec<Deduction<N>> {
    match full_house {
        Step::FullHouse { cell, value, .. } => vec![Deduction::Placement(*cell, *value)],
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, full_house: &Step<N>) -> String {
    match full_house {
        Step::FullHouse { house, cell, value: Candidate(value) } => format!(
            "Full House; {} is the last cell in {}, and must contain {}",
            grid.cell_name(*cell), grid.cell_set_name(house), value
        ),
        _ => unreachable!(),
    }
}

fn find_for_house<'a, const N: usize>(grid: &'a Grid<N>, house: &CellSet<N>) -> Option<Step<N>> {
    let empty_cells = grid.empty_cells_in(house);
    if empty_cells.len() == 1 {
        let cell = empty_cells.first().unwrap();
        if grid.num_candidates(cell) == 0 {
            Some(Step::NoCandidatesForCell { cell })
        } else {
            let value = grid.first_candidate(cell).unwrap();
            Some(Step::FullHouse { house: house.clone(), cell, value })
        }
    } else {
        None
    }
}
