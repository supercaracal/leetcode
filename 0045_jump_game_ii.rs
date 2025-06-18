fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2,3,1,1,4");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", jump(nums));
    Ok(())
}

// https://www.youtube.com/watch?v=dJ7sWiOoK7g
fn jump(nums: Vec<i32>) -> i32 {
    let mut cnt = 0;
    let mut l = 0;
    let mut r = 0;
    while l < nums.len() && r < nums.len() {
        let mut farthest = 0;
        for i in l..=r {
            farthest = farthest.max(i as i32 + nums[i]);
        }
        l = r + 1;
        r = farthest as usize;
        if l < nums.len() {
            cnt += 1;
        }
    }
    cnt
}
