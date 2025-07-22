fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,1,1,2,2,3");
    }
    let mut nums = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", remove_duplicates(&mut nums));
    println!("{nums:?}");
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut l = 0;
    let mut cnt = 0;
    let mut curr = i32::MAX;
    for r in 0..nums.len() {
        if nums[r] != curr {
            curr = nums[r];
            cnt = 0;
        }
        nums[l] = nums[r];
        cnt += 1;
        if cnt <= 2 {
            l += 1;
        }
    }
    l as i32
}
