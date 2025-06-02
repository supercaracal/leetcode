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
    if divisor == 1 {
        return dividend;
    }
    if divisor == -1 {
        if dividend == i32::MIN {
            return i32::MAX;
        } else {
            return dividend * -1;
        }
    }
    let mut q = 0;
    let mut n = if dividend > 0 {
        -1 * dividend
    } else {
        dividend
    };
    let m = if divisor > 0 { -1 * divisor } else { divisor };
    while n <= m {
        n -= m;
        q += 1;
    }
    if (dividend < 0) == (divisor < 0) {
        1 * q
    } else {
        -1 * q
    }
}
