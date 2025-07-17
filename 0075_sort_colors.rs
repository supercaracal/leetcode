fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2,0,2,1,1,0");
    }
    let mut nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    sort_colors(&mut nums);
    println!("{nums:?}");
    Ok(())
}

#[allow(clippy::ptr_arg)]
#[allow(clippy::needless_range_loop)]
fn sort_colors(nums: &mut Vec<i32>) {
    let mut c_zero = 0;
    let mut c_one = 0;
    let mut c_two = 0;
    for n in nums.iter() {
        match n {
            0 => c_zero += 1,
            1 => c_one += 1,
            2 => c_two += 1,
            _ => {}
        }
    }
    for i in 0..c_zero {
        nums[i] = 0;
    }
    for i in c_zero..(c_zero + c_one) {
        nums[i] = 1;
    }
    for i in (c_zero + c_one)..(c_zero + c_one + c_two) {
        nums[i] = 2;
    }
}
