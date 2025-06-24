fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: -2,1,-3,4,-1,2,1,-5,4");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", max_sub_array(nums));
    Ok(())
}

fn max_sub_array(nums: Vec<i32>) -> i32 {
    // TODO: solve
    println!("{nums:?}");
    0
}
