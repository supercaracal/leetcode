fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 4");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", count_and_say(n));
    Ok(())
}

fn count_and_say(n: i32) -> String {
    // TODO: solve
    println!("{n}");
    "!".to_string()
}
