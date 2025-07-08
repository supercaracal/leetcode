fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 11 1");
    }
    println!("{:?}", add_binary(args[1].clone(), args[2].clone()));
    Ok(())
}

fn add_binary(a: String, b: String) -> String {
    // TODO: solve
    println!("{a:?}, {b:?}");
    "".to_string()
}
