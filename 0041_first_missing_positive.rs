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
    for v in nums.iter_mut() {
        if *v == 1 {
            contains1 = true;
        }
        if *v < 1 || *v > (n as i32) {
            *v = 1;
        }
    }
    if !contains1 {
        return 1;
    }
    for i in 0..n {
        let j = nums[i].unsigned_abs() as usize;
        if j == n {
            nums[0] = -(nums[0].abs());
        } else {
            nums[j] = -(nums[j].abs());
        }
    }
    for (i, v) in nums.iter().enumerate() {
        if *v > 0 {
            return i as i32;
        }
    }
    if nums[0] > 0 {
        return n as i32;
    }
    n as i32 + 1
}
