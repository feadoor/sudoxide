use std::collections::HashSet;

use itertools::Itertools;

use crate::grid::Grid;
use crate::grid::candidate::{Candidate, CandidateSet};
use crate::grid::cell::{CellIdx, CellSet};

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum ChainNode<const N: usize> {
    Value { cell: CellIdx<N>, value: Candidate<N> },
    Group { cells: CellSet<N>, value: Candidate<N> },
    Als { cells: CellSet<N>, cells_with_value: CellSet<N>, value: Candidate<N> },
}

impl<const N: usize> ChainNode<N> {
    pub fn description(&self, grid: &Grid<N>) -> String {
        match self {
            ChainNode::Value { cell, value } => format!("{}{}", value.0, grid.cell_name(*cell)),
            ChainNode::Group { cells, value } => format!("{}{}", value.0, grid.cell_set_name(cells)),
            ChainNode::Als { cells, value, .. } => format!("{}{}", value.0, grid.cell_set_name(cells)),
        }
    }

    pub fn value(&self) -> Candidate<N> {
        match self {
            ChainNode::Value { value, .. } => *value,
            ChainNode::Group { value, .. } => *value,
            ChainNode::Als { value, .. } => *value,
        }
    }

    pub fn value_cells(&self) -> CellSet<N> {
        match self {
            ChainNode::Value { cell, .. } => CellSet::from_cells([*cell]),
            ChainNode::Group { cells, .. } => cells.clone(),
            ChainNode::Als { cells_with_value, .. } => cells_with_value.clone(),
        }
    }
}

pub fn is_weakly_linked<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { .. }, ChainNode::Value { .. }) => is_weakly_linked_value_value(grid, start_node, end_node),
        (ChainNode::Value { .. }, ChainNode::Group { .. }) => is_weakly_linked_value_group(grid, start_node, end_node),
        (ChainNode::Value { .. }, ChainNode::Als { .. }) => is_weakly_linked_value_als(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Value { .. }) => is_weakly_linked_group_value(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Group { .. }) => is_weakly_linked_group_group(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Als { .. }) => is_weakly_linked_group_als(grid, start_node, end_node),
        (ChainNode::Als { .. }, ChainNode::Value { .. }) => is_weakly_linked_als_value(grid, start_node, end_node),
        (ChainNode::Als { .. }, ChainNode::Group { .. }) => is_weakly_linked_als_group(grid, start_node, end_node),
        (ChainNode::Als { .. }, ChainNode::Als { .. }) => is_weakly_linked_als_als(grid, start_node, end_node),
    }
}

pub fn is_strongly_linked<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>, xy_chain: bool) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { .. }, ChainNode::Value { .. }) => is_strongly_linked_value_value(grid, start_node, end_node, xy_chain),
        (ChainNode::Value { .. }, ChainNode::Group { .. }) => is_strongly_linked_value_group(grid, start_node, end_node),
        (ChainNode::Value { .. }, ChainNode::Als { .. }) => false,
        (ChainNode::Group { .. }, ChainNode::Value { .. }) => is_strongly_linked_group_value(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Group { .. }) => is_strongly_linked_group_group(grid, start_node, end_node),
        (ChainNode::Group { .. }, ChainNode::Als { .. }) => false,
        (ChainNode::Als { .. }, ChainNode::Value { .. }) => false,
        (ChainNode::Als { .. }, ChainNode::Group { .. }) => false,
        (ChainNode::Als { .. }, ChainNode::Als { .. }) => is_strongly_linked_als_als(grid, start_node, end_node),
    }
}

pub fn bivalue_nodes<const N: usize>(grid: &Grid<N>) -> Vec<ChainNode<N>> {
    let mut nodes = Vec::new();
    for cell in grid.cells_with_n_candidates(2).iter() {
        let mut candidates = grid.candidates(cell).iter();
        let (candidate1, candidate2) = (candidates.next().unwrap(), candidates.next().unwrap());
        nodes.push(ChainNode::Value { cell, value: candidate1 });
        nodes.push(ChainNode::Value { cell, value: candidate2 });
    }
    nodes
}

pub fn value_nodes_for_candidate<const N: usize>(grid: &Grid<N>, value: Candidate<N>) -> Vec<ChainNode<N>> {
    grid.cells_with_candidate(value).iter().map(|cell| ChainNode::Value { cell, value }).collect()
}

pub fn group_nodes_for_candidate<const N: usize>(grid: &Grid<N>, value: Candidate<N>) -> Vec<ChainNode<N>> {
    let mut nodes = Vec::new();
    for (house1, house2) in grid.all_houses().iter().tuple_combinations() {
        let cells = grid.cells_with_candidate_in(&(house1 & house2), value);
        if cells.len() > 1 { nodes.push(ChainNode::Group { cells, value }); }
    }
    nodes
}

pub fn value_nodes<const N: usize>(grid: &Grid<N>) -> Vec<ChainNode<N>> {
    grid.empty_cells().iter().flat_map(|cell| {
        grid.candidates(cell).iter().map(move |value| ChainNode::Value { cell, value })
    }).collect()
}

pub fn group_nodes<const N: usize>(grid: &Grid<N>) -> Vec<ChainNode<N>> {
    grid.all_values().iter().flat_map(|value| group_nodes_for_candidate(grid, value)).collect()
}

pub fn als_nodes<const N: usize>(grid: &Grid<N>) -> Vec<ChainNode<N>> {
    let mut nodes = HashSet::new();
    for house in grid.all_houses() {
        let empty_cells = grid.empty_cells_in(house);
        for degree in 2 .. empty_cells.len() {
            for cells in empty_cells.iter().combinations(degree).map(CellSet::from_cells) {
                let candidates = CandidateSet::union(cells.iter().map(|cell| grid.candidates(cell)));
                if candidates.len() == degree + 1 {
                    for value in candidates.iter() {
                        let node = ChainNode::Als { cells: cells.clone(), cells_with_value: grid.cells_with_candidate_in(&cells, value), value };
                        if !nodes.contains(&node) { nodes.insert(node); }
                    }
                }
            }
        }
    }
    nodes.into_iter().collect()
}

fn is_weakly_linked_value_value<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { cell: start_cell, value: start_value }, ChainNode::Value { cell: end_cell, value: end_value }) => {
            if start_value == end_value { grid.neighbours(*start_cell).contains(*end_cell) }
            else { start_cell == end_cell }
        },
        _ => unreachable!(),
    }
}

fn is_weakly_linked_value_group<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { cell: start_cell, value: start_value }, ChainNode::Group { cells: end_cells, value: end_value }) => {
            start_value == end_value && grid.neighbours(*start_cell).contains_all(end_cells)
        },
        _ => unreachable!(),
    }
}

fn is_weakly_linked_value_als<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { cell: start_cell, value: start_value }, ChainNode::Als { cells_with_value: end_cells, value: end_value, .. }) => {
            if start_value == end_value { grid.neighbours(*start_cell).contains_all(end_cells) }
            else { &CellSet::from_cells([*start_cell]) == end_cells }
        },
        _ => unreachable!(),
    }
}

fn is_weakly_linked_group_value<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    is_weakly_linked_value_group(grid, end_node, start_node)
}

fn is_weakly_linked_group_group<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Group { cells: start_cells, value: start_value }, ChainNode::Group { cells:end_cells, value: end_value }) => {
            start_value == end_value && grid.common_neighbours(start_cells).contains_all(end_cells)
        },
        _ => unreachable!(),
    }
}

fn is_weakly_linked_group_als<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Group { cells: start_cells, value: start_value }, ChainNode::Als { cells_with_value: end_cells, value: end_value, .. }) => {
            start_value == end_value && grid.common_neighbours(start_cells).contains_all(end_cells)
        },
        _ => unreachable!(),
    }
}

fn is_weakly_linked_als_value<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    is_weakly_linked_value_als(grid, end_node, start_node)
}

fn is_weakly_linked_als_group<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    is_weakly_linked_group_als(grid, end_node, start_node)
}

fn is_weakly_linked_als_als<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Als { cells_with_value: start_cells, value: start_value, .. }, ChainNode::Als { cells_with_value: end_cells, value: end_value, .. }) => {
            if start_value == end_value { grid.common_neighbours(start_cells).contains_all(end_cells) }
            else { start_cells.len() == 1 && start_cells == end_cells }
        },
        _ => unreachable!(),
    }
}

fn is_strongly_linked_value_value<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>, xy_chain: bool) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { cell: start_cell, value: start_value }, ChainNode::Value { cell: end_cell, value: end_value }) => {
            if start_value == end_value && start_cell != end_cell && !xy_chain {
                let houses_to_consider = grid.all_houses_containing(&CellSet::from_cells([*start_cell, *end_cell]));
                houses_to_consider.iter().any(|&house| grid.cells_with_candidate_in(house, *start_value).len() == 2)
            } else {
                start_cell == end_cell && start_value != end_value && grid.num_candidates(*start_cell) == 2
            }
        },
        _ => unreachable!(),
    }
}

fn is_strongly_linked_value_group<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Value { cell: start_cell, value: start_value }, ChainNode::Group { cells: end_cells, value: end_value }) => {
            if start_value != end_value || end_cells.contains(*start_cell) { false }
            else {
                let mut involved_cells = end_cells.clone(); involved_cells.add_cell(*start_cell);
                grid.all_houses().iter().any(|house| house.contains(*start_cell) && involved_cells.contains_all(&grid.cells_with_candidate_in(house, *start_value)))
            }
        },
        _ => unreachable!(),
    }
}

fn is_strongly_linked_group_value<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    is_strongly_linked_value_group(grid, end_node, start_node)
}

fn is_strongly_linked_group_group<const N: usize>(grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Group { cells: start_cells, value: start_value }, ChainNode::Group { cells: end_cells, value: end_value }) => {
            if start_value != end_value || start_cells == end_cells { false }
            else {
                let involved_cells = start_cells | end_cells;
                grid.all_houses().iter().any(|house| house.intersects(start_cells) && involved_cells.contains_all(&grid.cells_with_candidate_in(house, *start_value)))
            }
        },
        _ => unreachable!(),
    }
}

fn is_strongly_linked_als_als<const N: usize>(_grid: &Grid<N>, start_node: &ChainNode<N>, end_node: &ChainNode<N>) -> bool {
    match (start_node, end_node) {
        (ChainNode::Als { cells: start_cells, value: start_value, .. }, ChainNode::Als { cells: end_cells, value: end_value, .. }) => {
            start_cells == end_cells && start_value != end_value
        },
        _ => unreachable!(),
    }
}
