fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 4");
    }
    let x = args[1].parse::<i32>().unwrap();
    println!("{:?}", my_sqrt(x));
    Ok(())
}

#[allow(clippy::comparison_chain)]
fn my_sqrt(x: i32) -> i32 {
    let (mut l, mut r) = (0, x);
    let mut res = 0;
    while l <= r {
        let m = l + ((r - l) / 2);
        match m.checked_mul(m) {
            Some(n) => {
                if n > x {
                    r = m - 1;
                } else if n < x {
                    l = m + 1;
                    res = m;
                } else {
                    return m;
                }
            }
            None => r = m - 1,
        }
    }
    res
}
