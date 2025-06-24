fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 4");
    }
    let n = args[1].parse::<i32>().unwrap();
    for row in solve_n_queens(n) {
        println!("{:?}", row);
    }
    Ok(())
}

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut ret = Vec::new();
    for i in 0..n {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        board[0][i as usize] = 'Q';
        if !backtrack(&mut board) {
            continue;
        }
        let ans = board
            .iter()
            .map(|row| {
                let mut s = String::with_capacity(row.len());
                for c in row {
                    s.push(*c);
                }
                s
            })
            .collect();
        ret.push(ans);
    }
    ret
}

fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
    if board.len() < 2 {
        return true;
    }
    for r in 1..board.len() {
        for c in 0..board.len() {
            if board[r][c] == 'Q' {
                continue;
            }
            if !is_safe(r, c, board) {
                continue;
            }
            board[r][c] = 'Q';
            if backtrack(board) {
                return true;
            }
            board[r][c] = '.';
        }
        if board[r].iter().all(|c| *c == '.') {
            return false;
        }
    }
    true
}

fn is_safe(r: usize, c: usize, board: &Vec<Vec<char>>) -> bool {
    if board[r][c] == 'Q' {
        return false;
    }
    for row in board.iter() {
        if row[c] == 'Q' {
            return false;
        }
    }
    for c in &board[r] {
        if *c == 'Q' {
            return false;
        }
    }
    let size = board.len();
    let mut i = r;
    let mut j = c;
    loop {
        if board[i][j] == 'Q' {
            return false;
        }
        if i == size - 1 || j == size - 1 {
            break;
        }
        i += 1;
        j += 1;
    }
    i = r;
    j = c;
    loop {
        if board[i][j] == 'Q' {
            return false;
        }
        if i == 0 || j == 0 {
            break;
        }
        i -= 1;
        j -= 1;
    }
    i = r;
    j = c;
    loop {
        if board[i][j] == 'Q' {
            return false;
        }
        if i == size - 1 || j == 0 {
            break;
        }
        i += 1;
        j -= 1;
    }
    i = r;
    j = c;
    loop {
        if board[i][j] == 'Q' {
            return false;
        }
        if i == 0 || j == size - 1 {
            break;
        }
        i -= 1;
        j += 1;
    }
    true
}
