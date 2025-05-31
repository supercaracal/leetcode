fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 10 3");
    }
    let devidend = args[1].parse::<i32>().unwrap();
    let divisor = args[2].parse::<i32>().unwrap();
    println!("{:?}", divide(devidend, divisor));
    Ok(())
}

fn divide(dividend: i32, divisor: i32) -> i32 {
    // TODO: solve
    dividend + divisor
}
