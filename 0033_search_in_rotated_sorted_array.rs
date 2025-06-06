fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 4,5,6,7,0,1,2 0");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", search(nums, target));
    Ok(())
}

// https://www.youtube.com/watch?v=U8XENwh8Oy8
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l <= r {
        let mid = (l + r) / 2;
        if target == nums[mid] {
            return mid as i32;
        }
        if nums[l] <= nums[mid] {
            if target > nums[mid] || target < nums[l] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        } else {
            if target < nums[mid] || target > nums[r] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
    }
    -1
}
