fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2,3,1,1,4");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", jump(nums));
    Ok(())
}

fn jump(nums: Vec<i32>) -> i32 {
    let mut jumps = Vec::new();
    backtrack(nums.as_ref(), 0, 0, &mut jumps);
    jumps.iter().min().map_or(-1, |n| *n)
}

fn backtrack(nums: &[i32], i: usize, n: i32, jumps: &mut Vec<i32>) {
    if i == nums.len() - 1 {
        jumps.push(n);
        return;
    }
    if i >= nums.len() || nums[i] == 0 {
        return;
    }
    for j in 1..=nums[i] {
        backtrack(nums, i + (j as usize), n + 1, jumps);
    }
}
