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

// https://www.youtube.com/watch?v=Ph95IHmRp5M
fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut col = HashSet::new();
    let mut pos_diag = HashSet::new();
    let mut neg_diag = HashSet::new();
    let mut ret = Vec::new();
    let mut board = vec![vec!['.'; n as usize]; n as usize];
    backtrack(
        0,
        &mut board,
        &mut ret,
        &mut col,
        &mut pos_diag,
        &mut neg_diag,
    );
    ret
}

use std::collections::HashSet;

fn backtrack(
    r: usize,
    board: &mut Vec<Vec<char>>,
    ret: &mut Vec<Vec<String>>,
    col: &mut HashSet<i32>,
    pos_diag: &mut HashSet<i32>,
    neg_diag: &mut HashSet<i32>,
) {
    if r == board.len() {
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
    for c in 0..board.len() {
        let co = c as i32;
        let pos = r as i32 + c as i32;
        let neg = r as i32 - c as i32;
        if col.contains(&co) || pos_diag.contains(&pos) || neg_diag.contains(&neg) {
            continue;
        }

        col.insert(co);
        pos_diag.insert(pos);
        neg_diag.insert(neg);
        board[r][c] = 'Q';
        backtrack(r + 1, board, ret, col, pos_diag, neg_diag);
        board[r][c] = '.';
        col.remove(&co);
        pos_diag.remove(&pos);
        neg_diag.remove(&neg);
    }
}
