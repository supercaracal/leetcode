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

// https://www.youtube.com/watch?v=4psV4SedHg0
fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    backtrack(board);
}

fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] != '.' {
                continue;
            }
            for n in '0'..'9' {
                if is_valid(board, r, c, n) {
                    board[r][c] = n;
                    if backtrack(board) {
                        return true;
                    }
                    board[r][c] = '.';
                }
            }
            return false;
        }
    }
    true
}

fn is_valid(board: &Vec<Vec<char>>, r: usize, c: usize, n: char) -> bool {
    for i in 0..9 {
        if board[r][i] == n
            || board[i][c] == n
            || board[3 * (r / 3) + i / 3][3 * (c / 3) + i % 3] == n
        {
            return false;
        }
    }
    true
}
