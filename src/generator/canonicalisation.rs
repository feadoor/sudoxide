pub fn minlex<const N: usize>(puzzle: &[usize]) -> Vec<usize> {
    
    let (mut reverse_lookup, mut seen, mut count) = (vec![0; N + 1], vec![false; N + 1], 0);
    for &clue in puzzle {
        if clue != 0 && !seen[clue] {
            seen[clue] = true;
            count += 1;
            reverse_lookup[clue] = count;
        }
    }

    let mut new_puzzle = vec![0; N * N];
    for (idx, &clue) in puzzle.iter().enumerate() {
        new_puzzle[idx] = reverse_lookup[clue];
    }

    new_puzzle
}
