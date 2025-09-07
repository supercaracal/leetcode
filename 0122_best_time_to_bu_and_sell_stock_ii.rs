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
    // TODO: optimize, simplify
    let mut holding = prices[0];
    let mut profit = 0;
    let mut total = 0;
    for v in prices.iter().skip(1) {
        println!("holding={holding}, profit={profit}, total={total}, v={v}");
        if &holding >= v {
            holding = *v;
            total += profit;
            profit = 0;
        } else if profit < v - holding {
            profit = v - holding;
        } else {
            holding = *v;
            total += profit;
            profit = 0;
        }
    }
    if profit > 0 {
        total += profit;
    }
    total
}
