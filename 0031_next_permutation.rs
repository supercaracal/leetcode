fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,2,3");
    }
    let mut nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    next_permutation(&mut nums);
    println!("{:?}", nums);
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn next_permutation(nums: &mut Vec<i32>) {
    if nums.is_empty() || nums.len() == 1 {
        return;
    }
    let mut i = nums.len() - 2;
    while nums[i + 1] <= nums[i] {
        if i == 0 {
            break;
        }
        i -= 1;
    }
    let mut j = nums.len() - 1;
    while nums[j] <= nums[i] {
        if j == 0 {
            break;
        }
        j -= 1;
    }
    nums.swap(i, j);
    i += 1;
    if i > j {
        i = 0;
    }
    j = nums.len() - 1;
    while i < j {
        nums.swap(i, j);
        i += 1;
        j -= 1;
    }
}
