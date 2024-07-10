mod full_house;
mod hidden_single;
mod naked_single;
mod pointing_claiming;
mod hidden_subset;
mod naked_subset;
mod fish;
mod y_wing;
mod w_wing;
mod xyz_wing;

use std::iter::empty;

use crate::grid::cell::{CellIdx, CellSet};
use crate::grid::candidate::{Candidate, CandidateSet};
use crate::grid::{Grid, House};

#[derive(Copy, Clone)]
pub enum Deduction<const N: usize> {
    Placement(CellIdx<N>, Candidate<N>),
    Elimination(CellIdx<N>, Candidate<N>),
    Contradiction,
}

impl<const N: usize> Deduction<N> {
    pub fn description(&self, grid: &Grid<N>) -> String {
        match self {
            Deduction::Placement(cell, value) => format!("{} placed in {}", value.0, grid.cell_name(*cell)),
            Deduction::Elimination(cell, value) => format!("{} eliminated from {}", value.0, grid.cell_name(*cell)),
            Deduction::Contradiction => format!("Contradiction!"),
        }
    }
}

pub enum Step<const N: usize> {
    NoCandidatesForCell { cell: CellIdx<N> },
    NoPlaceForCandidateInHouse { house: CellSet<N>, value: Candidate<N> },
    FullHouse { house: CellSet<N>, cell: CellIdx<N>, value: Candidate<N> },
    HiddenSingle { house: CellSet<N>, cell: CellIdx<N>, value: Candidate<N> },
    NakedSingle { cell: CellIdx<N>, value: Candidate<N> },
    PointingClaiming { house: CellSet<N>, neighbours: CellSet<N>, value: Candidate<N> },
    HiddenSubset { house: CellSet<N>, cells: CellSet<N>, values: CandidateSet<N> },
    NakedSubset { cells: CellSet<N>, values: CandidateSet<N> },
    Fish { base_type: House, base: CellSet<N>, cover: CellSet<N>, fins: CellSet<N>, value: Candidate<N>, },
    YWing { pivot: CellIdx<N>, pincer1: CellIdx<N>, pincer2: CellIdx<N>, value: Candidate<N> },
    WWing { pincer1: CellIdx<N>, pincer2: CellIdx<N>, house: CellSet<N>, covered_value: Candidate<N>, eliminated_value: Candidate<N> },
    XYZWing { pivot: CellIdx<N>, pincer1: CellIdx<N>, pincer2: CellIdx<N>, value: Candidate<N> },
}

#[derive(Copy, Clone)]
pub enum Strategy {
    FullHouse,
    HiddenSingle,
    NakedSingle,
    PointingClaiming,
    HiddenSubset(usize),
    NakedSubset(usize),
    Fish(usize),
    FinnedFish(usize),
    YWing,
    WWing,
    XYZWing,
}

pub fn all_strategies(n: usize) -> Vec<Strategy> {
    empty()
        .chain([Strategy::FullHouse, Strategy::HiddenSingle, Strategy::NakedSingle, Strategy::PointingClaiming])
        .chain((2 ..= n / 2).flat_map(|degree| [Strategy::NakedSubset(degree), Strategy::HiddenSubset(degree)]))
        .chain((2 ..= n / 2).map(|degree| Strategy::Fish(degree)))
        .chain([Strategy::YWing, Strategy::WWing, Strategy::XYZWing])
        .chain((2 ..= n / 2).map(|degree| Strategy::FinnedFish(degree)))
        .collect()
}

impl<const N: usize> Step<N> {

    pub fn deductions(&self, grid: &Grid<N>) -> Vec<Deduction<N>> {
        match self {
            Step::NoCandidatesForCell { .. } => vec![Deduction::Contradiction],
            Step::NoPlaceForCandidateInHouse { .. } => vec![Deduction::Contradiction],
            ref full_house @ Step::FullHouse { .. } => full_house::deductions(grid, full_house),
            ref hidden_single @ Step::HiddenSingle { .. } => hidden_single::deductions(grid, hidden_single),
            ref naked_single @ Step::NakedSingle { .. } => naked_single::deductions(grid, naked_single),
            ref pointing_claiming @ Step::PointingClaiming { .. } => pointing_claiming::deductions(grid, pointing_claiming),
            ref hidden_subset @ Step::HiddenSubset { .. } => hidden_subset::deductions(grid, hidden_subset),
            ref naked_subset @ Step::NakedSubset { .. } => naked_subset::deductions(grid, naked_subset),
            ref fish @ Step::Fish { .. } => fish::deductions(grid, fish),
            ref y_wing @ Step::YWing { .. } => y_wing::deductions(grid, y_wing),
            ref w_wing @ Step::WWing { .. } => w_wing::deductions(grid, w_wing),
            ref xyz_wing @ Step::XYZWing { .. } => xyz_wing::deductions(grid, xyz_wing),
        }
    }

    pub fn description(&self, grid: &Grid<N>) -> String {
        match self {
            Step::NoCandidatesForCell { cell } => format!("No candidates remain for cell {}", grid.cell_name(*cell)),
            Step::NoPlaceForCandidateInHouse { house, value: Candidate(value) } => format!("No place for {} in {}", value, grid.cell_set_name(house)),
            ref full_house @ Step::FullHouse { .. } => full_house::description(grid, full_house),
            ref hidden_single @ Step::HiddenSingle { .. } => hidden_single::description(grid, hidden_single),
            ref naked_single @ Step::NakedSingle { .. } => naked_single::description(grid, naked_single),
            ref pointing_claiming @ Step::PointingClaiming { .. } => pointing_claiming::description(grid, pointing_claiming),
            ref hidden_subset @ Step::HiddenSubset { .. } => hidden_subset::description(grid, hidden_subset),
            ref naked_subset @ Step::NakedSubset { .. } => naked_subset::description(grid, naked_subset),
            ref fish @ Step::Fish { .. } => fish::description(grid, fish),
            ref y_wing @ Step::YWing { .. } => y_wing::description(grid, y_wing),
            ref w_wing @ Step::WWing { .. } => w_wing::description(grid, w_wing),
            ref xyz_wing @ Step::XYZWing { .. } => xyz_wing::description(grid, xyz_wing),
        }
    }
}

impl Strategy {
    pub fn find_steps<'a, const N: usize>(&self, grid: &'a Grid<N>) -> Box<dyn Iterator<Item = Step<N>> + 'a> {
        match *self {
            Strategy::FullHouse => Box::new(full_house::find(grid)),
            Strategy::HiddenSingle => Box::new(hidden_single::find(grid)),
            Strategy::NakedSingle => Box::new(naked_single::find(grid)),
            Strategy::PointingClaiming => Box::new(pointing_claiming::find(grid)),
            Strategy::HiddenSubset(degree) => Box::new(hidden_subset::find(grid, degree)),
            Strategy::NakedSubset(degree) => Box::new(naked_subset::find(grid, degree)),
            Strategy::Fish(degree) => Box::new(fish::find(grid, degree, false)),
            Strategy::FinnedFish(degree) => Box::new(fish::find(grid, degree, true)),
            Strategy::YWing => Box::new(y_wing::find(grid)),
            Strategy::WWing => Box::new(w_wing::find(grid)),
            Strategy::XYZWing => Box::new(xyz_wing::find(grid)),
        }
    }
}
