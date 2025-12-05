use advent_25::get_input;

fn get_surrounds(grid: &Vec<Vec<char>>, current_row: isize, current_col: isize) -> Vec<(char, usize, usize)> {
    let mut surrounds: Vec<(char, usize, usize)> = Vec::new();

    for r in current_row-1..=current_row+1 {
        if r < 0 || r >= grid.len() as isize {
            continue;
        }
        let row: usize = r.try_into().unwrap();
        let row_chars = &grid[row];
        for c in current_col-1..=current_col+1 {
            if r == current_row && c == current_col {
                continue;
            }
            if c < 0 || c >= grid.len() as isize {
                continue;
            }
            let col: usize = c.try_into().unwrap();
            surrounds.push((row_chars[col], row, col));
        }
    }

    surrounds
}

fn run_iteration(grid: &Vec<Vec<char>>) -> (u32, Vec<Vec<char>>) {
    let mut removed_rolls: u32 = 0;
    let mut new_grid = grid.to_vec();
    for (r, row) in grid.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            if *item == "@".chars().next().unwrap() {
                let surrounds = get_surrounds(&grid, r.try_into().unwrap(), c.try_into().unwrap());

                let surrounding_rolls: Vec<&(char, usize, usize)> = surrounds.iter().filter(|item| item.0 == "@".chars().next().unwrap()).collect();
                if surrounding_rolls.len() < 4 {
                    removed_rolls += 1;
                    new_grid[r][c] = ".".chars().next().unwrap();
                }
            }
        }
    }

    (removed_rolls, new_grid)
}

fn main() -> Result<(), &'static str> {
    let input: Vec<String> = get_input(4);
    let mut grid: Vec<Vec<char>> = input.iter().map(|row| Vec::from_iter(row.chars())).collect();

    let (rolls_1, new_grid): (u32, Vec<Vec<char>>) = run_iteration(&grid);
    let mut rolls_2: u32 = rolls_1;
    grid = new_grid;


    loop {
        let (removed, new_grid) = run_iteration(&grid);
        if removed == 0 {
            break;
        }
        rolls_2 += removed;
        grid = new_grid;
    }

    println!("1: {rolls_1}");
    println!("2: {rolls_2}");

    Ok(())
}
