fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 3,2,2,3 3");
    }
    let mut nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let val = args[2].parse::<i32>().unwrap();
    println!("{:?}", remove_element(&mut nums, val));
    Ok(())
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // TODO: solve
    println!("nums={nums:?}, val={val:?}");
    0
}
