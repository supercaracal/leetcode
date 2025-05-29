fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,1,2");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", remove_duplicates(nums));
    Ok(())
}

fn remove_duplicates(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 1;
    }
    let mut nums = nums;
    let mut i = 0;
    for j in 1..nums.len() {
        if nums[i] == nums[j] {
            nums[j] = i32::MAX;
        } else {
            i += 1;
            if i != j {
                nums[i] = nums[j];
                nums[j] = i32::MAX;
            }
        }
    }
    println!("{nums:?}");
    (i + 1) as i32
}
