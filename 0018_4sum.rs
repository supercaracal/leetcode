fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,0,-1,0,-2,2 0");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", four_sum(nums, target));
    Ok(())
}

fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let size = nums.len();
    use std::cmp::Ordering;
    use std::collections::HashSet;
    let mut quads = HashSet::new();
    for i in 0..size {
        for j in (i + 1)..size {
            let mut l = j + 1;
            let mut r = size - 1;
            while l < r {
                let mut sum = 0i32;
                for n in [nums[i], nums[j], nums[l], nums[r]] {
                    match sum.checked_add(n) {
                        Some(s) => sum = s,
                        None => continue,
                    }
                }
                match sum.cmp(&target) {
                    Ordering::Greater => r -= 1,
                    Ordering::Less => l += 1,
                    Ordering::Equal => {
                        quads.insert(vec![nums[i], nums[j], nums[l], nums[r]]);
                        l += 1;
                        while l < r && nums[l - 1] == nums[l] {
                            l += 1;
                        }
                    }
                }
            }
        }
    }
    quads.iter().map(|e| e.to_owned()).collect()
}
