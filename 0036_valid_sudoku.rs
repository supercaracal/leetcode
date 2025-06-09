fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 5,3,.,.,7,.,.,.,.,6,.,.,1,9,5,.,.,.,.,9,8,.,.,.,.,6,.,8,.,.,.,6,.,.,.,3,4,.,.,8,.,3,.,.,1,7,.,.,.,2,.,.,.,6,.,6,.,.,.,.,2,8,.,.,.,.,4,1,9,.,.,5,.,.,.,.,8,.,.,7,9");
    }
    let nums: Vec<char> = args[1]
        .split(',')
        .map(|e| e.parse::<char>().unwrap())
        .collect();
    let board = nums.chunks(9).map(|r| r.to_vec()).collect();
    println!("{:?}", is_valid_sudoku(board));
    Ok(())
}

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut blks = HashMap::new();
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == '.' {
                continue;
            }
            let n = board[r][c].to_digit(10).unwrap() as usize;
            if rows[r].contains(&n) || cols[c].contains(&n) {
                return false;
            }
            rows[r].insert(n);
            cols[c].insert(n);
            let blk_idx = (r / 3, c / 3);
            if !blks.contains_key(&blk_idx) {
                blks.insert(blk_idx, HashSet::new());
            }
            if let Some(set) = blks.get_mut(&blk_idx) {
                if set.contains(&n) {
                    return false;
                }
                set.insert(n);
            }
        }
    }
    true
}
