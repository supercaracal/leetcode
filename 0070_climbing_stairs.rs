fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", climb_stairs(n));
    Ok(())
}

fn climb_stairs(n: i32) -> i32 {
    // TODO: solve
    n
}
