fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,2,3");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", permute(nums));
    Ok(())
}

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    solve(nums.as_ref())
}

// https://www.youtube.com/watch?v=FZe0UqISmUw
fn solve(nums: &[i32]) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
        return vec![vec![]];
    }
    let perms = solve(&nums[1..]);
    let mut res = vec![];
    for p in perms {
        for i in 0..=p.len() {
            let mut c = p.clone();
            c.insert(i, nums[0]);
            res.push(c);
        }
    }
    res
}
