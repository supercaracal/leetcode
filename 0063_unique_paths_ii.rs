fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 0,0,0,0,1,0,0,0,0 3");
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
    println!("{:?}", unique_paths_with_obstacles(matrix));
    Ok(())
}

fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut matrix = vec![vec![0; n]; m];
    for r in 0..m {
        for c in 0..n {
            if obstacle_grid[r][c] == 1 {
                matrix[r][c] = 0;
            } else if r == 0 && c == 0 {
                matrix[r][c] = 1;
            } else if r == 0 && c > 0 {
                matrix[r][c] = matrix[r][c - 1];
            } else if r > 0 && c == 0 {
                matrix[r][c] = matrix[r - 1][c];
            } else {
                matrix[r][c] = matrix[r - 1][c] + matrix[r][c - 1];
            }
        }
    }
    matrix[m - 1][n - 1]
}
