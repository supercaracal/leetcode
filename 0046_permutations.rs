fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,2,3");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", permute(nums));
    Ok(())
}

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // TODO: solve
    println!("{nums:?}");
    vec![vec![]]
}
