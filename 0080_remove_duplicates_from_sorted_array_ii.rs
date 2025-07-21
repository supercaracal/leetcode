fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,1,1,2,2,3");
    }
    let mut nums = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", remove_duplicates(&mut nums));
    Ok(())
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // TODO: solve
    nums[0]
}
