fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 4");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", solve_n_queens(n));
    Ok(())
}

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    // TODO: solve
    println!("{n}");
    vec![vec![]]
}
