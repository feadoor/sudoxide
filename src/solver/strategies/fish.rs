use itertools::Itertools;

use crate::grid::{Grid, House};
use crate::grid::candidate::Candidate;
use crate::grid::cell::CellSet;

use super::{Deduction, Step};

pub fn find<'a, const N: usize>(grid: &'a Grid<N>, degree: usize, finned: bool) -> impl Iterator<Item = Step<N>> + 'a {
    grid.all_values().into_iter().flat_map(move |value| {
        [House::Row, House::Column].into_iter().flat_map(move |base_type| {
            let candidate_positions = grid.cells_with_candidate(value);
            let all_base_sets = grid.group_by(&candidate_positions, base_type);
            all_base_sets.into_iter().combinations(degree).flat_map(move |base_sets|
                find_for_base_and_value(grid, degree, &candidate_positions, &base_sets, base_type, value, finned)
            )
        })
    })
}

pub fn deductions<const N: usize>(grid: &Grid<N>, fish: &Step<N>) -> Vec<Deduction<N>> {
    match fish {
        Step::Fish { base, cover, fins, value, .. } => {
            (grid.common_neighbours(fins) & cover & !base).iter().map(|cell| Deduction::Elimination(cell, *value)).collect()
        },
        _ => unreachable!(),
    }
}

pub fn description<const N: usize>(grid: &Grid<N>, fish: &Step<N>) -> String {
    match fish {
        Step::Fish { base_type, base, cover, fins, value } => {
            let base_houses = match base_type { 
                House::Row => grid.intersecting_rows(base), 
                House::Column => grid.intersecting_columns(base), 
                _ => unreachable!() 
            };
            let cover_houses = match base_type { 
                House::Row => grid.intersecting_columns(cover), 
                House::Column => grid.intersecting_rows(cover), 
                _ => unreachable!() 
            };
            format!(
                "{}{}; on value {} with base ({}), cover ({}){}{}",
                if fins.is_empty() { "" } else { "Finned " },
                fish_name(base_houses.len()),
                value.0,
                base_houses.iter().map(|house| grid.cell_set_name(house)).join(", "),
                cover_houses.iter().map(|house| grid.cell_set_name(house)).join(", "),
                if fins.is_empty() { "" } else { " and fins " },
                if fins.is_empty() { "".to_string() } else { grid.cell_set_name(fins) },
            )
        }
        _ => unreachable!(),
    }
}

fn find_for_base_and_value<const N: usize>(grid: &Grid<N>, degree: usize, candidate_positions: &CellSet<N>, base_sets: &[CellSet<N>], base_type: House, value: Candidate<N>, finned: bool) -> Vec<Step<N>> {
    
    let base_union = CellSet::union(base_sets);
    let cover_sets = match base_type {
        House::Row => grid.intersecting_columns(&base_union),
        House::Column => grid.intersecting_rows(&base_union),
        _ => unreachable!(),
    };

    if !finned && cover_sets.len() == degree {
        let cover_union = CellSet::union(cover_sets) & candidate_positions;
        if cover_union.intersects(!&base_union) {
            vec![Step::Fish { base_type, base: base_union, cover: cover_union, fins: CellSet::empty(), value }]
        } else {
            vec![]
        }
    }

    else if finned && cover_sets.len() > degree {
        let num_fins = cover_sets.len() - degree;
        let full_cover = CellSet::union(&cover_sets) & candidate_positions;
        let mut finned_fish = vec![];

        for ex_covers in cover_sets.iter().combinations(num_fins) {
            let uncovered = CellSet::union(ex_covers);
            let cover_union = &full_cover & !&uncovered;
            let fins = &base_union & &uncovered;
            if grid.common_neighbours(&fins).intersects(&(&cover_union & !&base_union)) {
                finned_fish.push(Step::Fish { base_type, base: base_union.clone(), cover: cover_union, fins: fins, value });
            }
        }

        finned_fish
    }

    else { vec![] }
}

fn fish_name<'a>(degree: usize) -> &'a str {
    match degree {
        2 => "X-Wing", 3 => "Swordfish", 4 => "Jellyfish",
        5 => "Squirmbag", 6 => "Whale", 7 => "Leviathan",
        _ => "Fish",
    }
}
