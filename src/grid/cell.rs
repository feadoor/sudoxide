use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use bitvec::prelude::*;

use super::candidate::{Candidate, CandidateSet};

#[derive(Copy, Clone, Debug)]
pub struct CellIdx<const N: usize>(pub usize);

impl<const N: usize> CellIdx<N> {

    pub fn from_row_and_col(row: usize, col: usize) -> Self {
        Self(row * N + col)
    }

    pub fn row(&self) -> usize {
        self.0 / N
    }

    pub fn col(&self) -> usize {
        self.0 % N
    }
}

#[derive(Clone)]
pub struct Cell<const N: usize> {
    value: Option<Candidate<N>>,
    candidates: CandidateSet<N>,
}

impl<const N: usize> Cell<N> {

    pub fn empty() -> Self {
        Self { value: None, candidates: CandidateSet::full() }
    }

    pub fn value(&self) -> Option<Candidate<N>> {
        self.value
    }

    pub fn set_value(&mut self, value: Candidate<N>) {
        self.value = Some(value);
        self.candidates = CandidateSet::empty();
    }

    pub fn eliminate_candidate(&mut self, value: Candidate<N>) {
        self.candidates.remove_value(value);
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    pub fn candidates(&self) -> &CandidateSet<N> {
        &self.candidates
    }

    pub fn num_candidates(&self) -> usize {
        self.candidates.len()
    }

    pub fn first_candidate(&self) -> Option<Candidate<N>> {
        self.candidates.first()
    }

    pub fn has_candidate(&self, value: Candidate<N>) -> bool {
        self.candidates.contains(value)
    }

    pub fn has_any_of_candidates(&self, values: &CandidateSet<N>) -> bool {
        self.candidates.intersects(values)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct CellSet<const N: usize> {
    cells: BitVec,
}

impl<const N: usize> CellSet<N> {
    
    pub fn empty() -> Self {
        Self { cells: bitvec![0; N * N] }
    }

    pub fn full() -> Self {
        Self { cells: bitvec![1; N * N] }
    }

    pub fn intersection<T, I: IntoIterator<Item = T>>(cell_sets: I) -> Self where Self: BitAndAssign<T> {
        let mut result = Self::full();
        for cell_set in cell_sets { result &= cell_set; }
        result
    }

    pub fn union<T, I: IntoIterator<Item = T>>(cell_sets: I) -> Self where Self: BitOrAssign<T> {
        let mut result = Self::empty();
        for cell_set in cell_sets { result |= cell_set; }
        result
    }

    pub fn from_cells<I: IntoIterator<Item = CellIdx<N>>>(from_cells: I) -> Self {
        let mut cells = bitvec![0; N * N];
        for CellIdx(idx) in from_cells { cells.set(idx, true); }
        Self { cells }
    }

    pub fn len(&self) -> usize {
        self.cells.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, cell: CellIdx<N>) -> bool {
        self.cells[cell.0]
    }

    pub fn intersects<T>(&self, other: T) -> bool where Self: BitAnd<T, Output = Self> {
        !(self.clone() & other).is_empty()
    }

    pub fn first(&self) -> Option<CellIdx<N>> {
        self.cells.first_one().map(CellIdx)
    }

    pub fn into_iter(&self) -> impl Iterator<Item = CellIdx<N>> + Clone {
        self.cells.iter_ones().collect::<Vec<_>>().into_iter().map(CellIdx)
    }

    pub fn iter(&self) -> impl Iterator<Item = CellIdx<N>> + Clone + '_ {
        self.cells.iter_ones().map(CellIdx)
    }

    pub fn filter<P: FnMut(&CellIdx<N>) -> bool>(&self, predicate: P) -> CellSet<N> {
        CellSet::from_cells(self.iter().filter(predicate))
    }

    pub fn remove_cell(&mut self, cell: CellIdx<N>) {
        self.cells.set(cell.0, false);
    }
}

impl<const N: usize> Not for CellSet<N> {
    type Output = CellSet<N>;

    fn not(self) -> Self::Output {
        CellSet { cells: !self.cells }
    }
}

impl<const N: usize> Not for &CellSet<N> {
    type Output = CellSet<N>;

    fn not(self) -> Self::Output {
        !self.clone()
    }
}

impl<const N: usize> BitOrAssign for CellSet<N> {

    fn bitor_assign(&mut self, rhs: Self) {
        self.cells |= rhs.cells;
    }
}

impl<const N: usize> BitOrAssign<&Self> for CellSet<N> {

    fn bitor_assign(&mut self, rhs: &Self) {
        self.cells |= &rhs.cells;
    }
}

impl<T: Copy, const N: usize> BitOrAssign<&T> for CellSet<N> where CellSet<N>: BitOrAssign<T> {
    
    fn bitor_assign(&mut self, rhs: &T) {
        self.bitor_assign(*rhs) 
    }
}

impl<const N: usize> BitAndAssign for CellSet<N> {

    fn bitand_assign(&mut self, rhs: Self) {
        self.cells &= rhs.cells;
    }
}

impl<const N: usize> BitAndAssign<&Self> for CellSet<N> {

    fn bitand_assign(&mut self, rhs: &Self) {
        self.cells &= &rhs.cells;
    }
}

impl<T: Copy, const N: usize> BitAndAssign<&T> for CellSet<N> where CellSet<N>: BitAndAssign<T> {
    
    fn bitand_assign(&mut self, rhs: &T) {
        self.bitand_assign(*rhs) 
    }
}

impl<const N: usize> BitAnd for CellSet<N> {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs; self
    }
}

impl<const N: usize> BitAnd<&Self> for CellSet<N> {
    type Output = Self;

    fn bitand(mut self, rhs: &Self) -> Self::Output {
        self &= rhs; self
    }
}

impl<const N: usize> BitAnd<CellSet<N>> for &CellSet<N> {
    type Output = CellSet<N>;

    fn bitand(self, mut rhs: CellSet<N>) -> Self::Output {
        rhs &= self; rhs
    }
}

impl<const N: usize> BitAnd for &CellSet<N> {
    type Output = CellSet<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.clone() & rhs
    }
}

impl<const N: usize> BitOr for CellSet<N> {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs; self
    }
}

impl<const N: usize> BitOr<&Self> for CellSet<N> {
    type Output = Self;

    fn bitor(mut self, rhs: &Self) -> Self::Output {
        self |= rhs; self
    }
}

impl<const N: usize> BitOr<CellSet<N>> for &CellSet<N> {
    type Output = CellSet<N>;

    fn bitor(self, mut rhs: CellSet<N>) -> Self::Output {
        rhs |= self; rhs
    }
}

impl<const N: usize> BitOr for &CellSet<N> {
    type Output = CellSet<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.clone() | rhs
    }
}
