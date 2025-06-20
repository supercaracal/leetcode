fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,2,3");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", permute_unique(nums));
    Ok(())
}

fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut p = Vec::with_capacity(nums.len());
    let mut ps = Vec::with_capacity((1..=nums.len()).product());
    let mut uniq = HashSet::new();
    let mut counter = HashMap::new();
    for n in nums.iter() {
        let c = counter.get(n).map_or(0, |v| *v);
        counter.insert(*n, c + 1);
        uniq.insert(*n);
    }
    backtrack(nums.as_ref(), &mut p, &mut ps, &mut counter, &uniq);
    ps
}

use std::collections::{HashMap, HashSet};

fn backtrack(
    nums: &[i32],
    p: &mut Vec<i32>,
    ps: &mut Vec<Vec<i32>>,
    counter: &mut HashMap<i32, i32>,
    uniq: &HashSet<i32>,
) {
    if p.len() == nums.len() {
        ps.push(p.clone());
        return;
    }
    for n in uniq.iter() {
        let mut cnt = 0;
        if let Some(c) = counter.get(n) {
            if *c == 0 {
                continue;
            }
            cnt = *c;
        }
        p.push(*n);
        counter.insert(*n, cnt - 1);
        backtrack(nums, p, ps, counter, uniq);
        counter.insert(*n, cnt);
        p.pop();
    }
}
