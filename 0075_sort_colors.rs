fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2,0,2,1,1,0");
    }
    let mut nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    sort_colors(&mut nums);
    println!("{nums:?}");
    Ok(())
}

fn sort_colors(nums: &mut Vec<i32>) {
    // TODO: solve
    println!("{nums:?}");
}
