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
    // TODO: solve
    println!("{candidates:?}, {target:?}");
    vec![vec![0]]
}
