fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 2.00000 10");
    }
    let x = args[1].parse::<f64>().unwrap();
    let n = args[2].parse::<i32>().unwrap();
    println!("{:?}", my_pow(x, n));
    Ok(())
}

fn my_pow(x: f64, n: i32) -> f64 {
    // TODO: solve
    println!("{x}^{n}");
    0.0
}
