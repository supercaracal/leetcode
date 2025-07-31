fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,2,2");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", subsets_with_dup(nums));
    Ok(())
}

fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut indices = Vec::new();
    let mut subsets = HashSet::new();
    backtrack(nums.as_ref(), 0, &mut indices, &mut subsets);
    subsets.into_iter().collect()
}

use std::collections::HashSet;

fn backtrack(
    nums: &[i32],
    start: usize,
    indices: &mut Vec<usize>,
    subsets: &mut HashSet<Vec<i32>>,
) {
    let mut subset = Vec::with_capacity(indices.len());
    for i in indices.iter() {
        subset.push(nums[*i]);
    }
    subset.sort();
    subsets.insert(subset);
    // TODO: figure out a bitwise way
    for i in (start..nums.len())
        .filter(|e| !indices.contains(e))
        .collect::<Vec<_>>()
    {
        indices.push(i);
        backtrack(nums, i + 1, indices, subsets);
        indices.pop();
    }
}
