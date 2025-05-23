fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 121");
    }
    let x = args[1].parse::<i32>().unwrap();
    println!("{:?}", is_palindrome(x));
    Ok(())
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut p = x;
    let mut y = 0;
    while p > 0 {
        y = y * 10 + p % 10;
        p /= 10;
    }
    x == y
}
