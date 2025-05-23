fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: -1,2,1,-4 1");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", three_sum_closest(nums, target));
    Ok(())
}

fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;
    let mut nums = nums.to_owned();
    nums.sort();
    let mut sum = i32::MAX;
    for (i, _n) in nums.iter().enumerate() {
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let s = nums[i] + nums[l] + nums[r];
            if (s - target).abs() < (sum - target).abs() {
                sum = s;
            }
            match s.cmp(&target) {
                Ordering::Greater => r -= 1,
                Ordering::Less => l += 1,
                Ordering::Equal => return sum,
            }
        }
    }
    sum
}
