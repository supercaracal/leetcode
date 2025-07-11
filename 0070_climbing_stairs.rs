fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", climb_stairs(n));
    Ok(())
}

fn climb_stairs(n: i32) -> i32 {
    let mut step = Vec::new();
    let mut steps = Vec::new();
    backtrack(n, &mut step, &mut steps);
    steps.len() as i32
}

fn backtrack(n: i32, step: &mut Vec<i32>, steps: &mut Vec<Vec<i32>>) {
    if n < 0 {
        return;
    }
    if n == 0 {
        steps.push(step.clone());
        return;
    }
    for i in 1..=2 {
        step.push(i);
        backtrack(n - i, step, steps);
        step.pop();
    }
}
