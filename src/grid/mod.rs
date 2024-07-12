use candidate::{Candidate, CandidateSet};
use cell::{Cell, CellIdx, CellSet};

use crate::solver::strategies::Deduction;

use std::fmt;

pub mod candidate;
pub mod cell;
mod geometry;
pub mod variants;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum House {
    Row,
    Column,
    Region,
}

#[derive(Clone)]
pub struct Grid<const N: usize> {
    cells: Vec<Cell<N>>,
    rows: Vec<CellSet<N>>,
    cols: Vec<CellSet<N>>,
    regions: Vec<CellSet<N>>,
    all_houses: Vec<CellSet<N>>,
    neighbours: Vec<CellSet<N>>,
}

impl<const N: usize> fmt::Display for Grid<N> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let dashes = "+".to_string() + &String::from_utf8(vec![b'-'; 3 * N]).unwrap() + "+";

        write!(f, "{}\n", dashes)?;

        for row in &self.rows {
            write!(f, "|")?;
            for value in row.iter().map(|ix| self.value(ix)) {
                write!(f, "{:^3}", if let Some(Candidate(value)) = value { value.to_string() } else { ".".to_string() })?;
            }
            write!(f, "|\n")?;
        }

        write!(f, "{}", dashes)
    }
}

impl<const N: usize> Grid<N> {

    pub fn empty(regions: Vec<CellSet<N>>, additional_neighbours: Vec<CellSet<N>>) -> Self {
        let cells = vec![Cell::empty(); N * N];
        let rows = Self::create_rows();
        let cols = Self::create_cols();
        let all_houses: Vec<_> = regions.iter().chain(rows.iter()).chain(cols.iter()).map(|cs| cs.clone()).collect();
        let neighbours = Self::create_neighbours(&all_houses, additional_neighbours);

        Self { cells, rows, cols, regions, all_houses, neighbours }
    }

    pub fn apply_deduction(&mut self, deduction: Deduction<N>) {
        match deduction {
            Deduction::Placement(cell, val) => self.place_value(cell, val),
            Deduction::Elimination(cell, val) => self.eliminate_candidate(cell, val),
            Deduction::Contradiction => {},
        }
    }

    pub fn place_value(&mut self, cell: CellIdx<N>, value: Candidate<N>) {
        self.cells[cell.0].set_value(value);
        for neighbour in self.neighbours[cell.0].clone().iter() {
            self.eliminate_candidate(neighbour, value);
        }
    }

    pub fn eliminate_candidate(&mut self, cell: CellIdx<N>, value: Candidate<N>) {
        self.cells[cell.0].eliminate_candidate(value);
    }

    pub fn is_solved(&self) -> bool {
        self.cells.iter().all(|cell| !cell.is_empty())
    }

    pub fn is_empty(&self, cell: CellIdx<N>) -> bool {
        self.cells[cell.0].is_empty()
    }

    pub fn value(&self, cell: CellIdx<N>) -> Option<Candidate<N>> {
        self.cells[cell.0].value()
    }

    pub fn candidates(&self, cell: CellIdx<N>) -> &CandidateSet<N> {
        self.cells[cell.0].candidates()
    }

    pub fn num_candidates(&self, cell: CellIdx<N>) -> usize {
        self.cells[cell.0].num_candidates()
    }

    pub fn first_candidate(&self, cell: CellIdx<N>) -> Option<Candidate<N>> {
        self.cells[cell.0].first_candidate()
    }

    pub fn has_candidate(&self, cell: CellIdx<N>, value: Candidate<N>) -> bool {
        self.cells[cell.0].has_candidate(value)
    }

    pub fn has_any_of_candidates(&self, cell: CellIdx<N>, values: &CandidateSet<N>) -> bool {
        self.cells[cell.0].has_any_of_candidates(values)
    }

    pub fn empty_cells(&self) -> CellSet<N> {
        self.empty_cells_in(&CellSet::full())
    }

    pub fn cells_with_candidate(&self, value: Candidate<N>) -> CellSet<N> {
        self.cells_with_candidate_in(&CellSet::full(), value)
    }

    pub fn cells_with_n_candidates(&self, num_candidates: usize) -> CellSet<N> {
        self.cells_with_n_candidates_in(&CellSet::full(), num_candidates)
    }

    pub fn empty_cells_in(&self, cells: &CellSet<N>) -> CellSet<N> {
        cells.filter(|&cell| self.is_empty(cell))
    }

    pub fn cells_with_candidate_in(&self, cells: &CellSet<N>, value: Candidate<N>) -> CellSet<N> {
        cells.filter(|&cell| self.has_candidate(cell, value))
    }

    pub fn cells_with_any_of_candidates_in(&self, cells: &CellSet<N>, values: &CandidateSet<N>) -> CellSet<N> {
        cells.filter(|&cell| self.has_any_of_candidates(cell, values))
    }

    pub fn cells_with_exact_candidates_in(&self, cells: &CellSet<N>, values: &CandidateSet<N>) -> CellSet<N> {
        cells.filter(|&cell| self.candidates(cell) == values)
    }

    pub fn cells_with_n_candidates_in(&self, cells: &CellSet<N>, num_candidates: usize) -> CellSet<N> {
        cells.filter(|&cell| self.num_candidates(cell) == num_candidates)
    }

    pub fn values_in(&self, cells: &CellSet<N>) -> CandidateSet<N> {
        CandidateSet::from_candidates(cells.iter().filter_map(|cell| self.value(cell)))
    }

    pub fn candidates_in(&self, cells: &CellSet<N>) -> CandidateSet<N> {
        CandidateSet::union(cells.iter().map(|cell| self.candidates(cell)))
    }

    pub fn value_placed_in(&self, cells: &CellSet<N>, value: Candidate<N>) -> bool {
        cells.iter().any(|cell| self.value(cell) == Some(value))
    }

    pub fn candidate_appears_in(&self, cells: &CellSet<N>, value: Candidate<N>) -> bool {
        cells.iter().any(|cell| self.candidates(cell).contains(value))
    }

    pub fn values_missing_from(&self, cells: &CellSet<N>) -> CandidateSet<N> {
        !self.values_in(cells)
    }

    fn create_rows() -> Vec<CellSet<N>> {
        (0 .. N).map(|r| CellSet::from_cells((0 .. N).map(|c| CellIdx::from_row_and_col(r, c)))).collect()
    }

    fn create_cols() -> Vec<CellSet<N>> {
        (0 .. N).map(|c| CellSet::from_cells((0 .. N).map(|r| CellIdx::from_row_and_col(r, c)))).collect()
    }

    fn create_neighbours(all_houses: &[CellSet<N>], mut neighbours: Vec<CellSet<N>>) -> Vec<CellSet<N>> {
        
        for cell in 0 .. N * N {
            for CellIdx(neighbour) in neighbours[cell].into_iter() {
                neighbours[neighbour].add_cell(CellIdx(cell));
            }
        }

        for house in all_houses {
            for CellIdx(cell) in house.iter() {
                neighbours[cell] |= house;
            }
        }

        for cell in 0 .. N * N {
            neighbours[cell].remove_cell(CellIdx(cell))
        }

        neighbours
    }
}
