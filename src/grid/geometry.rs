use super::candidate::{Candidate, CandidateSet};
use super::{Grid, House};
use super::cell::{CellIdx, CellSet};

impl <const N: usize> Grid<N> {

    pub fn cell_name(&self, cell: CellIdx<N>) -> String {
        format!("r{}c{}", cell.row() + 1, cell.col() + 1)
    }

    pub fn cell_set_name(&self, cell_set: &CellSet<N>) -> String {
        for (idx, row) in self.rows.iter().enumerate() {
            if cell_set == row {
                return format!("Row {}", idx + 1);
            }
        }

        for (idx, col) in self.cols.iter().enumerate() {
            if cell_set == col {
                return format!("Column {}", idx + 1);
            }
        }

        for (idx, region) in self.regions.iter().enumerate() {
            if cell_set == region {
                return format!("Region {}", idx + 1);
            }
        }

        format!("({})", cell_set.iter().map(|c| self.cell_name(c)).collect::<Vec<_>>().join(", "))
    }

    pub fn cells(&self) -> CellSet<N> {
        CellSet::full()
    }

    pub fn rows(&self) -> &[CellSet<N>] {
        &self.rows
    }

    pub fn columns(&self) -> &[CellSet<N>] {
        &self.cols
    }

    pub fn regions(&self) -> &[CellSet<N>] {
        &self.regions
    }

    pub fn all_houses(&self) -> &[CellSet<N>] {
        &self.all_houses
    }

    pub fn all_values(&self) -> CandidateSet<N> {
        CandidateSet::full()
    }

    pub fn neighbours(&self, cell: CellIdx<N>) -> &CellSet<N> {
        &self.neighbours[cell.0]
    }

    pub fn common_neighbours(&self, cells: &CellSet<N>) -> CellSet<N> {
        CellSet::intersection(cells.iter().map(|cell| self.neighbours(cell)))
    }

    pub fn rows_with_candidate(&self, value: Candidate<N>) -> Vec<&CellSet<N>> {
        self.rows.iter().filter(|row| self.candidate_appears_in(row, value)).collect()
    }

    pub fn columns_with_candidate(&self, value: Candidate<N>) -> Vec<&CellSet<N>> {
        self.cols.iter().filter(|col| self.candidate_appears_in(col, value)).collect()
    }

    pub fn regions_with_candidate(&self, value: Candidate<N>) -> Vec<&CellSet<N>> {
        self.regions.iter().filter(|region| self.candidate_appears_in(region, value)).collect()
    }

    pub fn all_houses_containing(&self, cells: &CellSet<N>) -> Vec<&CellSet<N>> {
        self.all_houses.iter().filter(|house| house.contains_all(cells)).collect()
    }

    pub fn intersecting_rows(&self, cells: &CellSet<N>) -> Vec<&CellSet<N>> {
        self.rows.iter().filter(|row| row.intersects(cells)).collect()
    }

    pub fn intersecting_columns(&self, cells: &CellSet<N>) -> Vec<&CellSet<N>> {
        self.cols.iter().filter(|row| row.intersects(cells)).collect()
    }

    pub fn group_by(&self, cells: &CellSet<N>, house_type: House) -> Vec<CellSet<N>> {
        let regions = match house_type { House::Row => &self.rows, House::Column => &self.cols, House::Region => &self.regions };
        regions.iter().map(|region| region & cells).filter(|group| !group.is_empty()).collect()
    }
}
