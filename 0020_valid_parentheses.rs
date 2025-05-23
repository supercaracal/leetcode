fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: ()");
    }
    println!("{:?}", is_valid(args[1].clone()));
    Ok(())
}

fn is_valid(s: String) -> bool {
    s.len() % 2 == 0
}
