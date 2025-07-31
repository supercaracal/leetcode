fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 12");
    }
    println!("{:?}", num_decodings(args[1].clone()));
    Ok(())
}

fn num_decodings(s: String) -> i32 {
    // TODO: solve
    println!("{s:?}");
    0
}
