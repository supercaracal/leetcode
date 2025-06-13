fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 10,1,2,7,6,1,5 8");
    }
    let candidates: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", combination_sum2(candidates, target));
    Ok(())
}

fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut comb = Vec::new();
    let mut combs = Vec::new();
    backtrack(&candidates, target, 0, &mut comb, &mut combs);
    combs
}

fn backtrack(
    candidates: &[i32],
    target: i32,
    start: usize,
    comb: &mut Vec<i32>,
    combs: &mut Vec<Vec<i32>>,
) {
    if target == 0 {
        combs.push(comb.clone());
        return;
    }
    for i in start..candidates.len() {
        if i > start && candidates[i] == candidates[i - 1] {
            continue;
        }
        let n = candidates[i];
        if n > target {
            break;
        }
        comb.push(n);
        backtrack(candidates, target - n, i + 1, comb, combs);
        comb.pop();
    }
}
