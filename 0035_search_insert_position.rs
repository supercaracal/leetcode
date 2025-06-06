fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,3,5,6 5");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", search_insert(nums, target));
    Ok(())
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // TODO: solve
    println!("{nums:?}, {target:?}");
    0
}
