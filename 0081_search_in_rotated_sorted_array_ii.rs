fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 2,5,6,0,0,1,2 0");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", search(nums, target));
    Ok(())
}

fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l <= r {
        let m = l + (r - l) / 2;
        if nums[m] == target {
            return true;
        }
        if nums[l] < nums[m] {
            if nums[l] <= target && target < nums[m] {
                if m == 0 {
                    return false;
                }
                r = m - 1;
            } else {
                if m == nums.len() - 1 {
                    return false;
                }
                l = m + 1;
            }
        } else if nums[l] > nums[m] {
            if nums[m] < target && target <= nums[r] {
                if m == nums.len() - 1 {
                    return false;
                }
                l = m + 1;
            } else {
                if m == 0 {
                    return false;
                }
                r = m - 1;
            }
        } else {
            l += 1;
        }
    }
    false
}
