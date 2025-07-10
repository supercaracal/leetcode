fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 4");
    }
    let x = args[1].parse::<i32>().unwrap();
    println!("{:?}", my_sqrt(x));
    Ok(())
}

fn my_sqrt(x: i32) -> i32 {
    // TODO: solve
    x
}
