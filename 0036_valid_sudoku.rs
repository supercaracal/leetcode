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
    for i in 0..9 {
        let mut row = [false; 10];
        let mut col = [false; 10];
        for j in 0..9 {
            if board[i][j] == '.' {
                continue;
            }
            let n = board[i][j].to_digit(10).unwrap() as usize;
            if row[n] {
                return false;
            }
            row[n] = true;
        }
        for j in 0..9 {
            if board[j][i] == '.' {
                continue;
            }
            let n = board[j][i].to_digit(10).unwrap() as usize;
            if col[n] {
                return false;
            }
            col[n] = true;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            let mut blk = [false; 10];
            for k in 0..3 {
                for l in 0..3 {
                    let x = i * 3 + k;
                    let y = j * 3 + l;
                    println!("{x:?},{y:?}");
                    if board[y][x] == '.' {
                        continue;
                    }
                    let n = board[y][x].to_digit(10).unwrap() as usize;
                    if blk[n] {
                        return false;
                    }
                    blk[n] = true;
                }
            }
        }
    }
    true
}
