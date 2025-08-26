fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 5");
    }
    let num_rows = args[1].parse::<i32>().unwrap();
    println!("{:?}", generate(num_rows));
    Ok(())
}

fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    // TODO: solve
    println!("{num_rows}");
    vec![]
}
