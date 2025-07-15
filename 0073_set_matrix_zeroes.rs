fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,1,1,1,0,1,1,1,1 3");
    }
    let size = args[2].parse::<usize>().unwrap();
    let mut matrix = Vec::with_capacity(size);
    for nums in args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .chunks(size)
    {
        let mut row = Vec::with_capacity(size);
        for n in nums {
            row.push(*n);
        }
        matrix.push(row);
    }
    for row in matrix.iter() {
        println!("{row:?}");
    }
    set_zeroes(&mut matrix);
    println!("---------------------------------------------");
    for row in matrix.iter() {
        println!("{row:?}");
    }
    Ok(())
}

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut row_zero = false;
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if matrix[r][c] != 0 {
                continue;
            }
            if r > 0 {
                matrix[r][0] = 0;
            } else {
                row_zero = true;
            }
            matrix[0][c] = 0;
        }
    }
    for r in 1..matrix.len() {
        for c in 1..matrix[0].len() {
            if matrix[r][0] == 0 || matrix[0][c] == 0 {
                matrix[r][c] = 0;
            }
        }
    }
    if matrix[0][0] == 0 {
        for r in 0..matrix.len() {
            matrix[r][0] = 0;
        }
    }
    if row_zero {
        for c in 0..matrix[0].len() {
            matrix[0][c] = 0;
        }
    }
}
