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
    // TODO: solve
    for row in obstacle_grid {
        println!("{row:?}");
    }
    0
}
