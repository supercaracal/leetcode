fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 3,2,2,3 3");
    }
    let mut nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let val = args[2].parse::<i32>().unwrap();
    println!("{:?}", remove_element(&mut nums, val));
    Ok(())
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() || (nums.len() == 1 && nums[0] == val) {
        return 0;
    }
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        if nums[i] == val && nums[j] != val {
            nums[i] = nums[j];
            nums[j] = val;
        }
        if nums[i] != val {
            i += 1;
        }
        if nums[j] == val {
            j -= 1;
        }
    }
    println!("{nums:?}");
    if i == 0 && nums.len() > 1 {
        return 0;
    }
    (i + 1) as i32
}
