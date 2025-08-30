fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 7,1,5,3,6,4");
    }
    let prices: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", max_profit(prices));
    Ok(())
}

fn max_profit(prices: Vec<i32>) -> i32 {
    // TODO: solve
    println!("{prices:?}");
    0
}
