use std::collections::{HashSet, VecDeque};

use crate::grid::{cell::CellSet, Grid};
use crate::grid::candidate::Candidate;
use crate::grid::cell::CellIdx;

use super::nodes::{self, ChainNode};
use super::super::Deduction;

#[derive(PartialEq, Eq)]
struct AicInference<const N: usize> {
    node: ChainNode<N>,
    negated: bool,
}

type AffectedCandidate<const N: usize> = (CellIdx<N>, Candidate<N>);
pub struct Aic<const N: usize> { chain: Vec<AicInference<N>>, pub is_loop: bool }

impl<const N: usize> AicInference<N> {
    fn description(&self, grid: &Grid<N>) -> String {
        match self.negated {
            true => format!("-{}", self.node.description(grid)),
            false => format!("+{}", self.node.description(grid)),
        }
    }
}

pub fn find_aics<'a, const N: usize>(grid: &'a Grid<N>, nodes: Vec<ChainNode<N>>, xy_chain: bool) -> impl Iterator<Item = Aic<N>> + 'a {
    AicSearcher::for_nodes(grid, nodes, xy_chain)
}

pub fn deductions<const N: usize>(grid: &Grid<N>, aic: &Aic<N>) -> Vec<Deduction<N>> {
    if aic.is_loop { loop_deductions(grid, &aic.chain) } else { chain_deductions(grid, &aic.chain) }
}

pub fn description<const N: usize>(grid: &Grid<N>, aic: &Aic<N>) -> String {
    let mut description = format!("{}", aic.chain[0].description(grid));
    for inference in aic.chain.iter().skip(1) {
        description.push_str(&format!(" --> {}", inference.description(grid)));
    }
    if aic.is_loop { description.push_str(" --> Loop"); }
    description
}

fn chain_deductions<const N: usize>(grid: &Grid<N>, chain: &[AicInference<N>]) -> Vec<Deduction<N>> {
    let (start_node, end_node) = (&chain[0].node, &chain[chain.len() - 1].node);
    let start_affected_candidates = find_affected_candidates(grid, start_node);
    let end_affected_candidates = find_affected_candidates(grid, end_node);
    start_affected_candidates.intersection(&end_affected_candidates).map(|&(cell, value)| Deduction::Elimination(cell, value)).collect()
}

fn loop_deductions<const N: usize>(grid: &Grid<N>, chain: &[AicInference<N>]) -> Vec<Deduction<N>> {
    let odd_affected_candidates = chain.iter().step_by(2)
        .map(|inference| find_affected_candidates(grid, &inference.node))
        .flatten().collect::<HashSet<_>>();
    let even_affected_candiates = chain.iter().skip(1).step_by(2)
        .map(|inference| find_affected_candidates(grid, &inference.node))
        .flatten().collect::<HashSet<_>>();
    odd_affected_candidates.intersection(&even_affected_candiates).map(|&(cell, value)| Deduction::Elimination(cell, value)).collect()
}

fn find_affected_candidates<const N: usize>(grid: &Grid<N>, node: &ChainNode<N>) -> HashSet<AffectedCandidate<N>> {
    let mut affected_candidates = HashSet::new();

    let (value, value_cells) = (node.value(), node.value_cells());
    let common_neighbours = CellSet::intersection(value_cells.iter().map(|cell| grid.neighbours(cell)));
    for cell in grid.cells_with_candidate_in(&common_neighbours, value).iter() {
        affected_candidates.insert((cell, value));
    }

    if value_cells.len() == 1 {
        let cell = value_cells.first().unwrap();
        for other_value in grid.candidates(cell).iter().filter(|&other| other != value) {
            affected_candidates.insert((cell, other_value));
        }
    }

    affected_candidates
}

struct AicSearcher<'a, const N: usize> {
    grid: &'a Grid<N>,
    nodes: Vec<ChainNode<N>>,
    adjacencies: Vec<Vec<usize>>,
    affected_candidates: Vec<HashSet<AffectedCandidate<N>>>,
    queue: VecDeque<(usize, usize)>,
    parents: Vec<Vec<usize>>,
}

impl<'a, const N: usize> AicSearcher<'a, N> {

    fn for_nodes(grid: &'a Grid<N>, nodes: Vec<ChainNode<N>>, xy_chain: bool) -> Self {
        let adjacencies = Self::create_adjacencies(grid, &nodes, xy_chain);
        let affected_candidates = nodes.iter().map(|node| find_affected_candidates(grid, node)).collect();
        
        let mut queue = VecDeque::new();
        let parents = vec![vec![usize::MAX; adjacencies.len()]; nodes.len()];

        for start_idx in 0 .. nodes.len() { 
            queue.push_back((start_idx, 2 * start_idx + 1));
        }

        Self { grid, nodes, adjacencies, affected_candidates, queue, parents }
    }

    fn create_adjacencies(grid: &'a Grid<N>, nodes: &[ChainNode<N>], xy_chain: bool) -> Vec<Vec<usize>> {
        let mut adjacencies = vec![vec![]; 2 * nodes.len()];
        for (start_idx, start_node) in nodes.iter().enumerate() {
            for (end_idx, end_node) in nodes.iter().enumerate().skip(start_idx) {
                if nodes::is_weakly_linked(grid, start_node, end_node) {
                    adjacencies[2 * start_idx].push(2 * end_idx + 1);
                    adjacencies[2 * end_idx].push(2 * start_idx + 1);
                }
                if nodes::is_strongly_linked(grid, start_node, end_node, xy_chain) {
                    adjacencies[2 * start_idx + 1].push(2 * end_idx);
                    adjacencies[2 * end_idx + 1].push(2 * start_idx);
                }
            }
        }
        adjacencies
    }

    fn create_chain(&self, start_idx: usize, end_idx: usize) -> Aic<N> {
        let mut chain = Vec::new(); let mut current_idx = end_idx;
        loop {
            chain.push(AicInference { node: self.nodes[current_idx / 2].clone(), negated: current_idx % 2 == 1 });
            if current_idx == start_idx { break; }
            current_idx = self.parents[start_idx / 2][current_idx];
        }
        chain.reverse();

        let is_loop = nodes::is_weakly_linked(self.grid, &self.nodes[end_idx / 2], &self.nodes[start_idx / 2]);
        Aic { chain, is_loop }
    }
}

impl<'a, const N: usize> Iterator for AicSearcher<'a, N> {
    type Item = Aic<N>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start_idx, current_idx)) = self.queue.pop_front() {

            for &next_idx in &self.adjacencies[current_idx] {
                if self.parents[start_idx][next_idx] == usize::MAX {
                    self.queue.push_back((start_idx, next_idx));
                    self.parents[start_idx][next_idx] = current_idx;
                }
            }

            if current_idx % 2 == 0 && !self.affected_candidates[current_idx / 2].is_disjoint(&self.affected_candidates[start_idx]) {
                return Some(self.create_chain(2 * start_idx + 1, current_idx));
            }
        }

        None
    }
}
