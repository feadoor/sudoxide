use crate::grid::Grid;
use crate::solver::strategies::{Deduction, Strategy};

pub fn steps_to_solve<const N: usize>(mut grid: Grid<N>, strategies: &[Vec<Strategy>]) -> Option<Vec<usize>> {

    let mut steps_taken = vec![0; strategies.len()];
    'outer: while !grid.is_solved() {

        for (idx, strategy_group) in strategies.iter().enumerate() {
            let moves = strategy_group.iter().flat_map(|strat| strat.find_steps(&grid));
            let deductions: Vec<Deduction<N>> = moves.flat_map(|mov| mov.deductions(&grid)).collect();
            if !deductions.is_empty() {
                for deduction in deductions {
                    if let Deduction::Contradiction = deduction { return None; }
                    else { grid.apply_deduction(deduction); }
                }
                steps_taken[idx] += 1; continue 'outer;
            }
        }

        break;
    }

    if grid.is_solved() { Some(steps_taken) } else { None }
}
