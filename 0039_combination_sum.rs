fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 2,7,11,15 9");
    }
    let candidates: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", combination_sum(candidates, target));
    Ok(())
}

use std::collections::HashSet;

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut combs = HashSet::new();
    let mut comb = Vec::new();
    backtrack(&candidates, target, &mut comb, &mut combs);
    let mut ret = Vec::new();
    for c in combs {
        ret.push(c);
    }
    ret
}

fn backtrack(candidates: &[i32], target: i32, comb: &mut Vec<i32>, combs: &mut HashSet<Vec<i32>>) {
    let sum: i32 = comb.iter().sum();
    if sum > target {
        return;
    }
    if sum == target {
        let mut c = comb.clone();
        c.sort();
        combs.insert(c);
        return;
    }
    for n in candidates {
        comb.push(*n);
        backtrack(candidates, target, comb, combs);
        comb.pop();
    }
}
