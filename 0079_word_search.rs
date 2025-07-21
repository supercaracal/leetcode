fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: A,B,C,E,S,F,C,S,A,D,E,E 4 ABCCED");
    }
    let size = args[2].parse::<usize>().unwrap();
    let mut board = Vec::new();
    let mut row = Vec::with_capacity(size);
    for c in args[1].chars() {
        if c == ',' {
            continue;
        }
        row.push(c);
        if row.len() == size {
            board.push(row);
            row = Vec::with_capacity(size);
        }
    }
    for row in board.iter() {
        println!("{row:?}");
    }
    println!("{}", exist(board, args[3].clone()));
    Ok(())
}

// TODO: optimize
fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();
    if board.len() == 1 && word.len() == 1 && board[0][0] == word[0] {
        return true;
    }
    let mut tracks = vec![vec![false; board[0].len()]; board.len()];
    for (y, row) in board.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if backtrack(y, x, &board, 0, &word, &mut tracks) {
                return true;
            }
        }
    }
    false
}

fn backtrack(
    y: usize,
    x: usize,
    board: &Vec<Vec<char>>,
    p: usize,
    word: &Vec<char>,
    tracks: &mut Vec<Vec<bool>>,
) -> bool {
    if p == word.len() {
        return true;
    }
    if tracks[y][x] {
        return false;
    }
    if board[y][x] != word[p] {
        tracks[y][x] = false;
        return false;
    }
    tracks[y][x] = true;
    if y > 0 && backtrack(y - 1, x, board, p + 1, word, tracks) {
        return true;
    }
    if x > 0 && backtrack(y, x - 1, board, p + 1, word, tracks) {
        return true;
    }
    if y < board.len() - 1 && backtrack(y + 1, x, board, p + 1, word, tracks) {
        return true;
    }
    if x < board[y].len() - 1 && backtrack(y, x + 1, board, p + 1, word, tracks) {
        return true;
    }
    tracks[y][x] = false;
    false
}
