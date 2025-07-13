fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: horse ros");
    }
    println!("{:?}", min_distance(args[1].clone(), args[2].clone()));
    Ok(())
}

fn min_distance(word1: String, word2: String) -> i32 {
    // TODO: solve
    println!("{word1:?} => {word2:?}");
    0
}
