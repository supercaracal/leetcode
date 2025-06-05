fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 4,5,6,7,0,1,2 0");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", search(nums, target));
    Ok(())
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    println!("{nums:?}, {target:?}");
    0
}
