fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: aa a");
    }
    println!("{:?}", is_match(args[1].clone(), args[2].clone()));
    Ok(())
}

fn is_match(s: String, p: String) -> bool {
    // TODO: solve
    println!("{s:?}, {p:?}");
    false
}
