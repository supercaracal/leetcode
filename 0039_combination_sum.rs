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

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut combs = Vec::new();
    let mut comb = Vec::new();
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
        let c = candidates[i];
        if c > target {
            break;
        }
        comb.push(c);
        backtrack(candidates, target - c, i, comb, combs);
        comb.pop();
    }
}
