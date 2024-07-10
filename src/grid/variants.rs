use itertools::iproduct;

use super::candidate::Candidate;
use super::Grid;
use super::cell::{CellIdx, CellSet};

use std::fmt;

pub enum GridParseError<const N: usize> {
    BadLength,
    Contradiction(CellIdx<N>),
}

impl<const N: usize> fmt::Display for GridParseError<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GridParseError::*;
        match *self {
            BadLength => write!(f, "The grid does not have the expected length"),
            Contradiction(pos) => write!(f, "The clue at position {} contradicts the others", pos.0),
        }
    }
}

impl<const N: usize> fmt::Debug for GridParseError<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

pub trait Mapper<const N: usize> {
    fn map_byte_to_candidate(byte: u8) -> Option<Candidate<N>>;
}

pub trait Classic<const N: usize> {
    fn classic_regions() -> Vec<CellSet<N>>;
    fn empty_classic() -> Grid<N> {
        let regions = Self::classic_regions();
        Grid::empty(regions, vec![CellSet::empty(); N * N])
    }
}

impl Classic<9> for Grid<9> {

    fn classic_regions() -> Vec<CellSet<9>> {
        iproduct!((0 .. 9).step_by(3), (0 .. 9).step_by(3))
            .map(|(r, c)| iproduct!(0 .. 3, 0 .. 3).map(move |(x, y)| CellIdx::from_row_and_col(r + x, c + y)))
            .map(CellSet::from_cells)
            .collect()
    }
}

impl Mapper<9> for Grid<9> {

    fn map_byte_to_candidate(byte: u8) -> Option<Candidate<9>> {
        match byte {
            b'1' ..= b'9' => Some(Candidate((byte - b'0') as usize)),
            _ => None,
        }
    }
}

impl<const N: usize> Grid<N> where Grid<N>: Mapper<N> {

    pub fn from_empty_grid_and_clues(mut empty_grid: Grid<N>, clues: &[Option<Candidate<N>>]) -> Result<Grid<N>, GridParseError<N>> {
        if clues.len() != N * N {
            return Err(GridParseError::BadLength);
        }

        for (idx, clue) in clues.iter().enumerate() {
            if clue.is_some() {
                if empty_grid.has_candidate(CellIdx(idx), clue.unwrap()) {
                    empty_grid.place_value(CellIdx(idx), clue.unwrap());
                } else {
                    return Err(GridParseError::Contradiction(CellIdx(idx)));
                }
            }
        }

        Ok(empty_grid)
    }

    pub fn from_empty_grid_and_string(empty_grid: Grid<N>, input: &str) -> Result<Grid<N>, GridParseError<N>> {
        let clues: Vec<_> = input.bytes().map(Self::map_byte_to_candidate).collect();
        Self::from_empty_grid_and_clues(empty_grid, &clues)
    }
}
