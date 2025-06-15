fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 2 3");
    }
    println!("{:?}", multiply(args[1].clone(), args[2].clone()));
    Ok(())
}

fn multiply(num1: String, num2: String) -> String {
    // TODO: solve
    num1 + &num2
}
