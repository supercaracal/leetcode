fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 5,7,7,8,8,10 8");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", search_range(nums, target));
    Ok(())
}

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // TODO: solve
    println!("{nums:?}, {target:?}");
    vec![]
}
