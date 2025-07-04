fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 3 7");
    }
    let m = args[1].parse::<i32>().unwrap();
    let n = args[2].parse::<i32>().unwrap();
    println!("{:?}", unique_paths(m, n));
    Ok(())
}

fn unique_paths(m: i32, n: i32) -> i32 {
    // 1  1  1  1  1  1  1
    // 1  2  3  4  5  6  7
    // 1  3  6 10 15 21 28
    let m = m as usize;
    let n = n as usize;
    let mut matrix = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 {
                matrix[i][j] = 1;
                continue;
            }
            matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1];
        }
    }
    matrix[m - 1][n - 1]
}
