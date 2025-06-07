fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 5,7,7,8,8,10 8");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", search_range(nums, target));
    Ok(())
}

// https://www.youtube.com/watch?v=4sQL7R5ySUU
fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices = Vec::with_capacity(2);
    indices.push(bin_search(&nums, target, true));
    indices.push(bin_search(&nums, target, false));
    indices
}

fn bin_search(nums: &[i32], target: i32, left_bias: bool) -> i32 {
    if nums.is_empty() {
        return -1;
    }
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut i = -1;
    while l <= r {
        let m = (l + r) / 2;
        if target < nums[m] {
            if m == 0 {
                break;
            }
            r = m - 1;
        } else if target > nums[m] {
            if m == nums.len() - 1 {
                break;
            }
            l = m + 1;
        } else {
            i = m as i32;
            if left_bias {
                if m == 0 {
                    break;
                }
                r = m - 1;
            } else {
                if m == nums.len() - 1 {
                    break;
                }
                l = m + 1;
            }
        }
    }
    i
}
