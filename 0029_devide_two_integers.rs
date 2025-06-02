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
    let s = (dividend < 0) == (divisor < 0);
    let dividend: i64 = if dividend > 0 {
        dividend as i64
    } else {
        -(dividend as i64)
    };
    let divisor: i64 = if divisor > 0 {
        divisor as i64
    } else {
        -(divisor as i64)
    };
    let mut q: i64 = 0;
    let mut r: i64 = 0;
    for i in (0..32).rev() {
        r <<= 1;
        r |= (dividend >> i) & 1;
        if r >= divisor {
            r -= divisor;
            q |= 1 << i;
        }
    }
    if s {
        let max: i64 = (1 << 31) - 1;
        q.min(max) as i32
    } else {
        let min: i64 = -(1 << 31);
        -q.max(min) as i32
    }
}
