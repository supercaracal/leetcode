fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", generate_matrix(n));
    Ok(())
}

fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    // TODO: solve
    println!("{n}");
    vec![]
}
