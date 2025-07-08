fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,2,3");
    }
    let digits: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", plus_one(digits));
    Ok(())
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    digits.reverse();
    let mut i = 0;
    digits[i] += 1;
    while digits[i] >= 10 {
        digits[i] = digits[i] - 10;
        if i < digits.len() - 1 {
            digits[i + 1] += 1;
        } else {
            digits.push(1);
        }
        i += 1;
    }
    digits.reverse();
    digits
}
