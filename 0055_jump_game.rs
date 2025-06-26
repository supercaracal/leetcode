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
    backtrack(0, &nums)
}

fn backtrack(i: usize, nums: &[i32]) -> bool {
    if i == nums.len() - 1 {
        return true;
    }
    if i >= nums.len() {
        return false;
    }
    if nums[i] == 0 {
        return false;
    }
    for j in 1..=nums[i] {
        if backtrack(i + j as usize, nums) {
            return true;
        }
    }
    return false;
}
