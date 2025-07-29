fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", gray_code(n));
    Ok(())
}

fn gray_code(n: i32) -> Vec<i32> {
    // TODO: solve
    vec![0; n as usize]
}
