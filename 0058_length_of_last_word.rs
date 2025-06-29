fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(r#"usage: "Hello World""#);
    }
    println!("{:?}", length_of_last_word(args[1].clone()));
    Ok(())
}

fn length_of_last_word(s: String) -> i32 {
    // TODO: solve
    println!("{s:?}");
    0
}
