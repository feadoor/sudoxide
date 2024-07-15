use itertools::Itertools;

use crate::grid::Grid;
use crate::grid::candidate::Candidate;
use crate::grid::cell::CellSet;

use super::{Deduction, Step, TurbotFlavour};

pub fn find_skyscrapers<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_values().into_iter().flat_map(move |value| {
        grid.rows_with_candidate(value).into_iter().tuple_combinations().chain(grid.columns_with_candidate(value).into_iter().tuple_combinations())
            .flat_map(move |(line1, line2)| find_for_bases_and_value(grid, TurbotFlavour::Skyscraper, line1, line2, value))
    })
}

pub fn find_kites<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_values().into_iter().flat_map(move |value| {
        grid.rows_with_candidate(value).into_iter()
            .cartesian_product(grid.columns_with_candidate(value).into_iter())
            .filter(move |&(row, col)| !grid.candidate_appears_in(&(row & col), value))
            .flat_map(move |(row, col)| find_for_bases_and_value(grid, TurbotFlavour::TwoStringKite, row, col, value))
    })
}

pub fn find_rectangles<'a, const N: usize>(grid: &'a Grid<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_values().into_iter().flat_map(move |value| {
        grid.rows_with_candidate(value).into_iter().chain(grid.columns_with_candidate(value).into_iter())
            .cartesian_product(grid.regions_with_candidate(value).into_iter())
            .filter(|&(line, region)| (line & region).is_empty())
            .flat_map(move |(line, region)| find_for_bases_and_value(grid, TurbotFlavour::EmptyRectangle, line, region, value))
    })
}

pub fn deductions<const N: usize>(grid: &Grid<N>, turbot: &Step<N>) -> Vec<Deduction<N>> {
    match turbot {
        Step::TurbotFish { base1, base2, cover, value, .. } => {
            let base_cells = grid.cells_with_candidate_in(&(base1 | base2), *value);
            let cover_cells = grid.cells_with_candidate_in(cover, *value);
            let elimination_cells = grid.common_neighbours(&(base_cells & !cover_cells));
            grid.cells_with_candidate_in(&elimination_cells, *value).iter().map(|cell| Deduction::Elimination(cell, *value)).collect()
        },
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, turbot: &Step<N>) -> String {
    match turbot {
        Step::TurbotFish { flavour, base1, base2, cover, value } => format!(
            "{}; {} in {} and {}, linked by {}",
            name(*flavour), value.0, grid.cell_set_name(base1), grid.cell_set_name(base2), grid.cell_set_name(cover)
        ),
        _ => unreachable!(),
    }
}

fn name<'a>(flavour: TurbotFlavour) -> &'a str {
    match flavour {
        TurbotFlavour::Skyscraper => "Skyscraper",
        TurbotFlavour::TwoStringKite => "2-String Kite",
        TurbotFlavour::EmptyRectangle => "Empty Rectangle",
    }
}

fn find_for_bases_and_value<'a, const N: usize>(grid: &'a Grid<N>, flavour: TurbotFlavour, base1: &'a CellSet<N>, base2: &'a CellSet<N>, value: Candidate<N>) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_houses().iter().flat_map(move |cover| {
        let base_cells = grid.cells_with_candidate_in(&(base1 | base2), value);
        let cover_cells = grid.cells_with_candidate_in(cover, value);
        let elimination_cells = grid.common_neighbours(&(base_cells & !cover_cells));
        
        if grid.candidate_appears_in(&elimination_cells, value) {
            Some(Step::TurbotFish { flavour, base1: base1.clone(), base2: base2.clone(), cover: cover.clone(), value })
        } else {
            None
        }
    })
}
