fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3,3,5,0,0,3,1,4");
    }
    let prices: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", max_profit(prices));
    Ok(())
}

// 3,3,5,0,0,3,1,4
// 1,2,3,4,5
// 7,6,4,3,1
// 1,2,4,2,5,7,2,4,9,0
// 6,1,3,2,4,7
// 1,2,4,7
// 8,3,6,2,8,8,8,4,2,0,7,2,9,4,9
fn max_profit(prices: Vec<i32>) -> i32 {
    // TODO: fix
    if prices.len() == 2 {
        if prices[0] < prices[1] {
            return prices[1] - prices[0];
        } else {
            return 0;
        }
    }
    let mut l = 0;
    let mut r = prices.len() - 1;
    let mut l_bought = &prices[l];
    let mut r_sold = &prices[r];
    let mut l_max_profit = 0;
    let mut r_max_profit = 0;
    while l <= r {
        l_bought = l_bought.min(&prices[l]);
        r_sold = r_sold.max(&prices[r]);
        l_max_profit = l_max_profit.max(&prices[l] - l_bought);
        r_max_profit = r_max_profit.max(r_sold - &prices[r]);
        println!(
            "l={}, r={}, l_max={}, r_max={}",
            prices[l], prices[r], l_max_profit, r_max_profit
        );
        if l + 1 < prices.len() && &prices[l] < &prices[l + 1] {
            l += 1;
        } else if r > 0 && &prices[r - 1] < &prices[r] {
            r -= 1;
        } else if l + 1 == prices.len() {
            break;
        } else if r == 0 {
            break;
        } else if l_max_profit < r_max_profit {
            l += 1;
        } else {
            r -= 1;
        }
    }
    l_max_profit + r_max_profit
}
