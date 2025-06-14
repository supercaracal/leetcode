fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,2,0");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", first_missing_positive(nums));
    Ok(())
}

fn first_missing_positive(nums: Vec<i32>) -> i32 {
    // TODO: use cycle sort
    let mut nums = nums;
    let n = nums.len();
    let mut contains1 = false;
    for i in 0..n {
        if nums[i] == 1 {
            contains1 = true;
        }
        if nums[i] < 1 || nums[i] > (n as i32) {
            nums[i] = 1;
        }
    }
    if !contains1 {
        return 1;
    }
    for i in 0..n {
        let j = nums[i].abs() as usize;
        if j == n {
            nums[0] = -(nums[0].abs());
        } else {
            nums[j] = -(nums[j].abs());
        }
    }
    for i in 1..n {
        if nums[i] > 0 {
            return i as i32;
        }
    }
    if nums[0] > 0 {
        return n as i32;
    }
    n as i32 + 1
}
