use itertools::Itertools;

use crate::grid::Grid;
use crate::grid::cell::{CellIdx, CellSet};

use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.cells_with_n_candidates(2).into_iter().tuple_combinations()
        .filter(|&(p1, p2)| grid.candidates(p1) == grid.candidates(p2))
        .flat_map(|(p1, p2)| grid.all_houses().iter().filter(move |house| !house.contains(p1) && !house.contains(p2)).map(move |house| (p1, p2, house)))
        .flat_map(|(p1, p2, house)| find_for_pincers_and_house(grid, p1, p2, house))
}

pub fn deductions<const N: usize>(grid: &Grid<N>, w_wing: &Step<N>) -> Vec<Deduction<N>> {
    match w_wing {
        Step::WWing { pincer1, pincer2, eliminated_value, .. } => grid
            .cells_with_candidate_in(&(grid.neighbours(*pincer1) & grid.neighbours(*pincer2)), *eliminated_value)
            .iter().map(|cell| Deduction::Elimination(cell, *eliminated_value)).collect(),
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, w_wing: &Step<N>) -> String {
    match w_wing {
        Step::WWing { pincer1, pincer2, house, covered_value, eliminated_value } => format!(
            "W-Wing; pincers ({}, {}) cover {} in {}, and so eliminate {} from common neighbours",
            grid.cell_name(*pincer1), grid.cell_name(*pincer2), covered_value.0, grid.cell_set_name(house), eliminated_value.0,
        ),
        _ => unreachable!(),
    }
}

fn find_for_pincers_and_house<'a, const N: usize>(grid: &'a Grid<N>, pincer1: CellIdx<N>, pincer2: CellIdx<N>, house: &'a CellSet<N>) -> impl Iterator<Item = Step<N>> + 'a {
    let mut candidates = grid.candidates(pincer1).iter();
    let (candidate1, candidate2) = (candidates.next().unwrap(), candidates.next().unwrap());
    let common_neighbours = grid.neighbours(pincer1) & grid.neighbours(pincer2);
    let unseen_cells = house & !(grid.neighbours(pincer1) | grid.neighbours(pincer2));
    
    [(candidate1, candidate2), (candidate2, candidate1)].into_iter()
        .filter(move |&(covered, _)| !grid.value_placed_in(&unseen_cells, covered) && !grid.candidate_appears_in(&unseen_cells, covered))
        .filter(move |&(_, eliminated)| grid.candidate_appears_in(&common_neighbours, eliminated))
        .map(move |(covered_value, eliminated_value)| Step::WWing { pincer1, pincer2, house: house.clone(), covered_value, eliminated_value })
}
