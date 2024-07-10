use std::collections::HashSet;

use itertools::Itertools;
use rand::prelude::*;

use crate::grid::candidate::Candidate;
use crate::grid::Grid;
use crate::grid::cell::CellIdx;

use super::brute_force::BruteForceSolver;
use super::canonicalisation::minlex;

type Pattern<const N: usize> = Vec<CellIdx<N>>;
type Puzzle = Vec<usize>;

pub struct PatternPuzzlesIterator<const N: usize> {
    starting_grid: Grid<N>,
    canonicalise: bool,
    brute_force_solver: BruteForceSolver<N>,
    seed_stack: Vec<Puzzle>,
    iteration_queue: Vec<Puzzle>,
    seen_puzzles: HashSet<Puzzle>,
    pattern: Pattern<N>,
}

impl<const N: usize> PatternPuzzlesIterator<N> {

    pub fn for_empty_grid_and_pattern(empty_grid: Grid<N>, pattern: Pattern<N>) -> Self {
        loop {
            if let Some(puzzle) = PatternPuzzlesIterator::random_seed(&empty_grid, &pattern) {
                let brute_force_solver = BruteForceSolver::for_empty_grid(&empty_grid);
                return Self {
                    starting_grid: empty_grid,
                    canonicalise: true,
                    brute_force_solver,
                    seed_stack: vec![puzzle],
                    iteration_queue: vec![],
                    seen_puzzles: HashSet::new(),
                    pattern,
                };
            }
        }
    }

    pub fn for_starting_grid_and_pattern(starting_grid: Grid<N>, pattern: Pattern<N>) -> Self {
        loop {
            if let Some(puzzle) = PatternPuzzlesIterator::random_seed(&starting_grid, &pattern) {
                let brute_force_solver = BruteForceSolver::for_starting_grid(&starting_grid);
                return Self {
                    starting_grid,
                    canonicalise: false,
                    brute_force_solver,
                    seed_stack: vec![puzzle],
                    iteration_queue: vec![],
                    seen_puzzles: HashSet::new(),
                    pattern,
                }
            }
        }
    }

    fn random_seed(starting_grid: &Grid<N>, pattern: &Pattern<N>) -> Option<Puzzle> {
        let mut puzzle = starting_grid.cells().iter().map(|c| starting_grid.value(c).map(|candidate| candidate.0).unwrap_or(0)).collect();
        for &cell in pattern {
            let valid_clues = PatternPuzzlesIterator::valid_clues(starting_grid, &puzzle, cell);
            if valid_clues.is_empty() { return None; }
            else { puzzle[cell.0] = *valid_clues.choose(&mut thread_rng()).unwrap(); }
        }
        Some(puzzle)
    }

    fn valid_clues(starting_grid: &Grid<N>, puzzle: &Puzzle, cell: CellIdx<N>) -> Vec<usize> {
        let mut valid = vec![false; N + 1];
        for Candidate(candidate) in starting_grid.candidates(cell).iter() {
            valid[candidate] = true;
        }
        if let Some(Candidate(value)) = starting_grid.value(cell) {
            valid[value] = true;
        }
        for CellIdx(neighbour) in starting_grid.neighbours(cell).iter() {
            valid[puzzle[neighbour]] = false;
        }
        (1 ..= N).filter(|&v| valid[v]).collect()
    }
}

impl<const N: usize> Iterator for PatternPuzzlesIterator<N> {
    type Item = Puzzle;

    fn next(&mut self) -> Option<Puzzle> {
        
        if let Some(puzzle) = self.iteration_queue.pop() {
            self.seed_stack.push(puzzle.clone());
            return Some(puzzle);
        }

        loop {

            while self.seed_stack.is_empty() {
                if let Some(seed) = Self::random_seed(&self.starting_grid, &self.pattern) {
                    self.seed_stack.push(seed);
                }
            }

            let current_puzzle = self.seed_stack.pop().unwrap();
            let mut next_puzzles = Vec::new();
            for (&clue1, &clue2) in self.pattern.iter().tuple_combinations() {

                let mut puzzle = current_puzzle.clone();
                puzzle[clue1.0] = 0; puzzle[clue2.0] = 0;

                let (poss1, poss2) = (Self::valid_clues(&self.starting_grid, &puzzle, clue1), Self::valid_clues(&self.starting_grid, &puzzle, clue2));
                for &c1 in &poss1 { puzzle[clue1.0] = c1;
                    for &c2 in &poss2 { puzzle[clue2.0] = c2;
                        let canonical_puzzle = if self.canonicalise { minlex::<N>(&puzzle) } else { puzzle.clone() };
                        if !self.seen_puzzles.contains(&canonical_puzzle) && self.brute_force_solver.has_unique_solution(&canonical_puzzle) {
                            self.seen_puzzles.insert(canonical_puzzle.clone());
                            next_puzzles.push(canonical_puzzle);
                        }
                    }
                }
            }

            next_puzzles.shuffle(&mut thread_rng());
            self.iteration_queue.append(&mut next_puzzles);

            if let Some(puzzle) = self.iteration_queue.pop() { return Some(puzzle); }
        }
    }
}
