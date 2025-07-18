fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 4 2");
    }
    let n = args[1].parse::<i32>().unwrap();
    let k = args[2].parse::<i32>().unwrap();
    println!("{:?}", combine(n, k));
    Ok(())
}

fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    // TODO: optimize
    let k = k as usize;
    let nums: Vec<i32> = (1..=n).collect();
    let mut comb = Vec::with_capacity(k);
    let mut combs = Vec::new();
    combination(k, 0, &nums, &mut comb, &mut combs);
    combs
}

fn combination(
    k: usize,
    i: usize,
    nums: &Vec<i32>,
    comb: &mut Vec<i32>,
    combs: &mut Vec<Vec<i32>>,
) {
    if comb.len() == k {
        combs.push(comb.clone());
        return;
    }
    for j in i..nums.len() {
        comb.push(nums[j]);
        combination(k, j + 1, nums, comb, combs);
        comb.pop();
    }
}
