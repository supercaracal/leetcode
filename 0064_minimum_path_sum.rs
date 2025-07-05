fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,3,1,1,5,1,4,2,1 3");
    }
    let size = args[2].parse::<usize>().unwrap();
    let mut grid = Vec::with_capacity(size);
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
        grid.push(row);
    }
    println!("{:?}", min_path_sum(grid));
    Ok(())
}

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    // TODO: solve
    for row in grid {
        println!("{row:?}");
    }
    0
}
