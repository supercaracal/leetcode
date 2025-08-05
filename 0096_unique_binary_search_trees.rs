fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", num_trees(n));
    Ok(())
}

fn num_trees(n: i32) -> i32 {
    // TODO: solve
    n
}
