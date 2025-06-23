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
    if x == 0.0 {
        return 0.0;
    }
    if n == 0 {
        return 1.0;
    }
    let m = n.checked_abs().map_or(i32::MAX, |v| v);
    let mut ret = my_pow(x, m / 2);
    ret *= ret;
    if m % 2 == 1 {
        ret *= x;
    }
    if n.checked_abs().map_or(1, |_| 0) == 1 {
        ret *= x;
    }
    if n.is_positive() {
        ret
    } else {
        1.0 / ret
    }
}
