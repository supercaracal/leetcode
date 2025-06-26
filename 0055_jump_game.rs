fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2,3,1,1,4");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", can_jump(nums));
    Ok(())
}

fn can_jump(nums: Vec<i32>) -> bool {
    // TODO: solve
    println!("{nums:?}");
    false
}
