fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2,3,1,1,4");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", can_jump(nums));
    Ok(())
}

// https://www.youtube.com/watch?v=Yan0cv2cLy8
fn can_jump(nums: Vec<i32>) -> bool {
    let mut r = nums.len() - 1;
    for i in (0..nums.len() - 1).rev() {
        if i as i32 + nums[i] >= r as i32 {
            r = i;
        }
    }
    r == 0
}
