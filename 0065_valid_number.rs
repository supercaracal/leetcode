fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 0");
    }
    println!("{:?}", is_number(args[1].clone()));
    Ok(())
}

fn is_number(s: String) -> bool {
    // TODO: solve
    println!("{s:?}");
    false
}
