fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", generate_parenthesis(n));
    Ok(())
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    // TODO: impl
    Vec::with_capacity(n as usize)
}
