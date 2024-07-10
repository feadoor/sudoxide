use crate::grid::Grid;
use crate::grid::candidate::Candidate;
use crate::grid::cell::CellIdx;

type Cell = usize;
type House = usize;
type DigitMask = usize;

struct ConstantData<const N: usize> {
    num_houses: usize,
    cells_for_house: Vec<Vec<Cell>>,
    houses_for_cell: Vec<Vec<House>>,
    neighbours_for_cell: Vec<Vec<Cell>>,
    start_state: BoardState,
}

#[derive(Clone)]
struct BoardState {
    cells: Vec<DigitMask>,
    cells_remaining: usize,
    solved_in_house: Vec<DigitMask>,
    solution: Vec<usize>,
}

impl BoardState {

    fn for_empty_grid<const N: usize>(grid: &Grid<N>) -> Self {
        Self {
            cells: vec![(1 << N) - 1; N * N],
            cells_remaining: N * N,
            solved_in_house: vec![0; grid.all_houses().len()],
            solution: vec![0; N * N],
        }
    }

    fn for_starting_grid<const N: usize>(grid: &Grid<N>) -> Self {
        Self {
            cells: (0 .. N * N).map(|cell| Self::maskify(grid, CellIdx(cell))).collect(),
            cells_remaining: N * N,
            solved_in_house: vec![0; grid.all_houses().len()],
            solution: vec![0; N * N],
        }
    }

    fn maskify<const N: usize>(grid: &Grid<N>, cell: CellIdx<N>) -> DigitMask {
        let mut mask = 0;
        for Candidate(candidate) in grid.candidates(cell).iter() {
            mask |= 1 << (candidate - 1);
        }
        if let Some(Candidate(value)) = grid.value(cell) {
            mask |= 1 << (value - 1);
        }
        mask
    }
}

#[derive(Copy, Clone)]
struct Placement {
    cell: Cell,
    mask: DigitMask,
}

#[derive(Copy, Clone)]
struct Guess {
    cell: Cell,
    mask: DigitMask,
    remaining: DigitMask,
}

pub struct BruteForceSolver<const N: usize> {
    
    constants: ConstantData<N>,

    invalid: bool,
    finished: bool,

    board: BoardState,
    board_stack: Vec<BoardState>,
    solution_count: usize,

    placement_queue: Vec<Placement>,
    guess_stack: Vec<Guess>,
}

impl<const N: usize> BruteForceSolver<N> {

    pub fn for_empty_grid(grid: &Grid<N>) -> Self {
        let constants = Self::init_constants(grid);
        Self::create(constants)
    }

    pub fn for_starting_grid(grid: &Grid<N>) -> Self {
        let mut constants = Self::init_constants(grid);
        constants.start_state = BoardState::for_starting_grid(grid);
        Self::create(constants)
    }

    pub fn has_unique_solution(&mut self, clues: &[usize]) -> bool {
        self.run(clues, 2);
        self.solution_count == 1
    }

    #[cfg(test)]
    pub fn solution(&mut self, clues: &[usize]) -> Vec<usize> {
        self.run(clues, 1);
        self.board.solution.iter().map(|x| x.trailing_zeros() as usize + 1).collect()
    }

    fn reset(&mut self) {
        self.invalid = false;
        self.finished = false;
        self.board = self.constants.start_state.clone();
        self.board_stack.clear();
        self.solution_count = 0;
        self.placement_queue.clear();
        self.guess_stack.clear();

        for cell in 0 .. N * N {
            let mask = self.board.cells[cell];
            let remaining = mask.count_ones();
            if remaining == 1 { self.enqueue_placement(cell, mask); }
            else if remaining == 0 { self.invalid = true; }
        }
    }

    fn prepare_with_clues(&mut self, clues: &[usize]) {
        self.reset();
        for (cell, &clue) in clues.iter().enumerate() {
            if clue != 0 {
                self.enqueue_placement(cell, 1 << (clue - 1));
            }
        }
    }

    fn run(&mut self, clues: &[usize], max_solutions: usize) {
        self.prepare_with_clues(clues);
        while !self.finished {
            self.process_queue();
            if self.board.cells_remaining > 0 && !self.invalid {
                self.check_hidden_singles();
                if self.placement_queue.is_empty() { self.guess(); }
            }
            else if self.invalid { self.backtrack(); }
            else if self.board.cells_remaining == 0 {
                self.solution_count += 1;
                if self.solution_count >= max_solutions { break; }
                self.backtrack();
            }
        }
    }

    fn process_queue(&mut self) {
        while !self.placement_queue.is_empty() && !self.invalid {
            let placement = self.placement_queue.pop().unwrap();
            self.place(placement);
            for neighbour_idx in 0 .. self.constants.neighbours_for_cell[placement.cell].len() {
                let neighbour = self.constants.neighbours_for_cell[placement.cell][neighbour_idx];
                if self.board.cells[neighbour] & placement.mask != 0 {
                    self.board.cells[neighbour] ^= placement.mask;
                    let neighbour_mask = self.board.cells[neighbour];
                    let remaining = neighbour_mask.count_ones();
                    if remaining == 1 { self.enqueue_placement(neighbour, neighbour_mask); }
                    else if remaining == 0 { self.invalid = true; return; }
                }
            }
        }
    }

    fn check_hidden_singles(&mut self) {
        for house in 0 .. self.constants.num_houses {
            let (mut at_least_once, mut more_than_once) = (0, 0);
            
            for idx in 0 .. N {
                let mask = self.board.cells[self.constants.cells_for_house[house][idx]];
                more_than_once |= at_least_once & mask;
                at_least_once |= mask;
            }

            if at_least_once | self.board.solved_in_house[house] != (1 << N) - 1 {
                self.invalid = true;
                return;
            }

            let mut exactly_once = at_least_once & !more_than_once;
            if exactly_once != 0 {
                for idx in 0 .. N {
                    let cell = self.constants.cells_for_house[house][idx];
                    let mask = self.board.cells[cell] & exactly_once;
                    if mask != 0 {
                        if mask.count_ones() > 1 {
                            self.invalid = true;
                            return;
                        }
                        self.enqueue_placement(cell, mask);
                        exactly_once ^= mask; if exactly_once == 0 { break; }
                    }
                }
            }
        }
    }

    fn get_best_cell_to_guess(&mut self) -> Option<Cell> {
        let (mut best_cell, mut best_digits) = (0, N + 1);
        for cell in 0 .. N * N {
            let digits = self.board.cells[cell].count_ones() as usize;
            if digits > 1 && digits < best_digits {
                best_cell = cell; best_digits = digits;
                if digits == 2 { break; }
            }
        }
        if best_digits == N + 1 { None } else { Some(best_cell) }
    }

    fn get_guess_for_cell(&mut self, cell: Cell) -> Guess {
        let cell_mask = self.board.cells[cell];
        let guess_mask = 1 << cell_mask.trailing_zeros();
        let leftovers = cell_mask ^ guess_mask;
        Guess { cell, mask: guess_mask, remaining: leftovers } 
    }

    fn guess(&mut self) {
        if let Some(best_cell) = self.get_best_cell_to_guess() {
            let guess = self.get_guess_for_cell(best_cell);
            self.board_stack.push(self.board.clone());
            self.guess_stack.push(guess);
            self.enqueue_placement(best_cell, guess.mask);
        } else {
            self.invalid = true;
        }
    }

    fn backtrack(&mut self) {
        if !self.board_stack.is_empty() {
            self.board = self.board_stack.pop().unwrap();
            self.placement_queue.clear();
            let guess = self.guess_stack.pop().unwrap();
            if guess.remaining.count_ones() > 1 {
                self.board.cells[guess.cell] = guess.remaining;
            } else {
                self.enqueue_placement(guess.cell, guess.remaining);
            }
            self.invalid = false;
        } else {
            self.finished = true;
        }
    }

    fn enqueue_placement(&mut self, cell: Cell, mask: DigitMask) {
        self.placement_queue.push(Placement { cell, mask });
    }

    fn place(&mut self, placement: Placement) {
        if self.board.cells[placement.cell] != 0 {
            
            let mask = placement.mask;
            if self.board.cells[placement.cell] & mask == 0 {
                self.invalid = true;
                return;
            }

            self.board.cells[placement.cell] = 0;
            for &house in &self.constants.houses_for_cell[placement.cell] {
                self.board.solved_in_house[house] |= mask;
            }

            self.board.solution[placement.cell] = placement.mask;
            self.board.cells_remaining -= 1;
        }

        else if self.board.solution[placement.cell] != placement.mask {
            self.invalid = true;
        }
    }

    fn create(constants: ConstantData<N>) -> Self {
        Self { board: constants.start_state.clone(), constants, invalid: false, finished: false, board_stack: Vec::new(), solution_count: 0, placement_queue: Vec::new(), guess_stack: Vec::new() }
    }

    fn init_constants(grid: &Grid<N>) -> ConstantData<N> {
        ConstantData {
            num_houses: Self::num_houses(grid),
            cells_for_house: Self::cells_for_house(grid),
            houses_for_cell: Self::houses_for_cell(grid),
            neighbours_for_cell: Self::neighbours_for_cell(grid),
            start_state: BoardState::for_empty_grid(grid),
        }
    }

    fn num_houses(grid: &Grid<N>) -> usize {
        grid.all_houses().len()
    }

    fn cells_for_house(grid: &Grid<N>) -> Vec<Vec<House>> {
        grid.all_houses().iter().map(|house| house.iter().map(|cell| cell.0).collect()).collect()
    }

    fn houses_for_cell(grid: &Grid<N>) -> Vec<Vec<Cell>> {
        let mut houses_for_cell = vec![vec![]; N * N];
        for (idx, house) in grid.all_houses().iter().enumerate() {
            for CellIdx(cell) in house.iter() {
                houses_for_cell[cell].push(idx);
            }
        }
        houses_for_cell
    }

    fn neighbours_for_cell(grid: &Grid<N>) -> Vec<Vec<Cell>> {
        (0 .. N * N).map(|cell| grid.neighbours(CellIdx(cell)).iter().map(|cell| cell.0).collect()).collect()
    }
}

#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    use crate::grid::Grid;
    use crate::grid::candidate::Candidate;
    use crate::grid::cell::CellIdx;
    use crate::grid::variants::{Classic, Mapper};

    use super::BruteForceSolver;

    fn check_solution<const N: usize>(grid: &Grid<N>, solution: &[usize]) {
        for house in grid.all_houses() {
            for Candidate(value) in grid.all_values().iter() {
                assert!(house.iter().any(|CellIdx(cell)| solution[cell] == value))
            }
        }
    }

    #[test]
    fn test_brute_force_solves() {

        let empty_classic = Grid::<9>::empty_classic();
        let mut solver = BruteForceSolver::for_empty_grid(&empty_classic);

        let file = File::open("brute_force_grids.txt").expect("Input file not present");
        let lines = BufReader::new(file).lines().map(|l| l.expect("Error reading from file"));
        for line in lines.filter(|l| !l.is_empty() && !l.starts_with("//")) {
            let clues = line.bytes().map(Grid::<9>::map_byte_to_candidate).map(|value| value.map(|Candidate(v)| v).unwrap_or(0)).collect::<Vec<_>>();
            assert!(solver.has_unique_solution(&clues));
            check_solution(&empty_classic, &solver.solution(&clues));
        }
    }
}
