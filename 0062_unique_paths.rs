fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 3 7");
    }
    let m = args[1].parse::<i32>().unwrap();
    let n = args[2].parse::<i32>().unwrap();
    println!("{:?}", unique_paths(m, n));
    Ok(())
}

fn unique_paths(m: i32, n: i32) -> i32 {
    // TODO: solve
    m + n
}
