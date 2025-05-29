fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,1,2");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", remove_duplicates(nums));
    Ok(())
}

fn remove_duplicates(nums: Vec<i32>) -> i32 {
    // TODO: solve
    nums.len() as i32
}
