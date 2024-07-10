use crate::grid::Grid;
use crate::grid::cell::CellIdx;

use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.cells().into_iter().flat_map(|cell| find_for_cell(grid, cell))
}

pub fn deductions<const N: usize>(_grid: &Grid<N>, naked_single: &Step<N>) -> Vec<Deduction<N>> {
    match naked_single {
        Step::NakedSingle { cell, value } => vec![Deduction::Placement(*cell, *value)],
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, naked_single: &Step<N>) -> String {
    match naked_single {
        Step::NakedSingle { cell, value } => format!(
            "Naked Single; {} can only contain {}",
            grid.cell_name(*cell), value.0
        ),
        _ => unreachable!(),
    }
}

fn find_for_cell<const N: usize>(grid: &Grid<N>, cell: CellIdx<N>) -> Option<Step<N>> {
    if grid.num_candidates(cell) == 0 && grid.is_empty(cell) {
        Some(Step::NoCandidatesForCell { cell })
    } else if grid.num_candidates(cell) == 1 {
        let value = grid.first_candidate(cell).unwrap();
        Some(Step::NakedSingle { cell, value })
    } else {
        None
    }
}
