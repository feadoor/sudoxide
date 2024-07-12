mod aic;
mod nodes;

pub use aic::Aic;

use nodes::{als_nodes, bivalue_nodes, group_nodes, group_nodes_for_candidate, value_nodes, value_nodes_for_candidate};

use crate::grid::Grid;

use super::{Deduction, Step};

pub fn find_xy_chains<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    let nodes = bivalue_nodes(grid);
    aic::find_aics(grid, nodes, true).map(|aic| Step::XYChain { aic })
}

pub fn find_x_chains<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_values().into_iter().flat_map(|value| {
        let mut nodes = value_nodes_for_candidate(grid, value);
        nodes.append(&mut group_nodes_for_candidate(grid, value));
        aic::find_aics(grid, nodes, false).map(|aic| Step::XChain { aic })
    })
}

pub fn find_aics<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    let mut nodes = value_nodes(grid);
    nodes.append(&mut group_nodes(grid));
    aic::find_aics(grid, nodes, false).map(|aic| Step::Aic { aic })
}

pub fn find_als_aics<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    let mut nodes = value_nodes(grid);
    nodes.append(&mut group_nodes(grid));
    nodes.append(&mut als_nodes(grid));
    aic::find_aics(grid, nodes, false).map(|aic| Step::AlsAic { aic })
}

pub fn deductions<const N: usize>(grid: &Grid<N>, chain_step: &Step<N>) -> Vec<Deduction<N>> {
    match chain_step {
        Step::XYChain { aic } => aic::deductions(grid, aic),
        Step::XChain { aic } => aic::deductions(grid, aic),
        Step::Aic { aic } => aic::deductions(grid, aic),
        Step::AlsAic { aic } => aic::deductions(grid, aic),
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, chain_step: &Step<N>) -> String {
    match chain_step {
        Step::XYChain { aic } => format!("XY-Chain; {}", aic::description(grid, aic)),
        Step::XChain { aic } => format!("X-Chain; {}", aic::description(grid, aic)),
        Step::Aic { aic } => format!("AIC; {}", aic::description(grid, aic)),
        Step::AlsAic { aic } => format!("ALS-AIC; {}", aic::description(grid, aic)),
        _ => unreachable!(),
    }
}
