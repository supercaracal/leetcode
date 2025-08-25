fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: rabbbit rabbit");
    }
    println!("{:?}", num_distinct(args[1].clone(), args[2].clone()));
    Ok(())
}

fn num_distinct(s: String, t: String) -> i32 {
    // TODO: solve
    println!("{s:?}, {t:?}");
    0
}
