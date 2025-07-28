fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: great regeat");
    }
    println!("{:?}", is_scramble(args[1].clone(), args[2].clone()));
    Ok(())
}

fn is_scramble(s1: String, s2: String) -> bool {
    // TODO: solve
    s1 == s2
}
