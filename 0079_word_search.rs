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
    println!("{}", exist(board, args[3].clone()));
    Ok(())
}

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    // TODO: solve
    for row in board {
        println!("{row:?}");
    }
    println!("{word:?}");
    false
}
