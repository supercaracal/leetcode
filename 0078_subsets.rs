fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,2,3");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", subsets(nums));
    Ok(())
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut subset = Vec::new();
    let mut subsets = Vec::new();
    backtrack(0, nums.as_ref(), &mut subset, &mut subsets);
    subsets
}

fn backtrack(i: usize, nums: &[i32], subset: &mut Vec<i32>, subsets: &mut Vec<Vec<i32>>) {
    subsets.push(subset.clone());
    if subset.len() == nums.len() {
        return;
    }
    for j in i..nums.len() {
        subset.push(nums[j]);
        backtrack(j + 1, nums, subset, subsets);
        subset.pop();
    }
}
