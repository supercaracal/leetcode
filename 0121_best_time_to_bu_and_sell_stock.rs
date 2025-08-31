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

//  3  2  6  5  0  3
//    -1  4 -1 -5  3
//
//  7  1  5  3  6  4
//    -6  4 -2  3 -2
fn max_profit(prices: Vec<i32>) -> i32 {
    // TODO: optimize
    let mut profit = 0;
    let mut max_profit = 0;
    for i in 0..prices.len() {
        if i == 0 {
            continue;
        }
        let diff = prices[i] - prices[i - 1];
        if profit < 0 && diff > 0 {
            profit = diff;
        } else {
            profit += diff;
        }
        max_profit = max_profit.max(profit);
    }
    max_profit
}
