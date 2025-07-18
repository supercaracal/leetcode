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
    // TODO: solve
    println!("n={n}, k={k}");
    vec![]
}
