fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2,1,5,6,2,3");
    }
    let heights: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", largest_rectangle_area(heights));
    Ok(())
}

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    // TODO: solve
    println!("{heights:?}");
    0
}
