fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 5,3,.,.,7,.,.,.,.,6,.,.,1,9,5,.,.,.,.,9,8,.,.,.,.,6,.,8,.,.,.,6,.,.,.,3,4,.,.,8,.,3,.,.,1,7,.,.,.,2,.,.,.,6,.,6,.,.,.,.,2,8,.,.,.,.,4,1,9,.,.,5,.,.,.,.,8,.,.,7,9");
    }
    let nums: Vec<char> = args[1]
        .split(',')
        .map(|e| e.parse::<char>().unwrap())
        .collect();
    let mut board = nums.chunks(9).map(|r| r.to_vec()).collect();
    solve_sudoku(&mut board);
    for row in board.iter() {
        println!("{row:?}");
    }
    Ok(())
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    // TODO: solve
    for row in board {
        println!("{row:?}");
    }
}
