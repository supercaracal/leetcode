fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 'A man, a plan, a canal: Panama'");
    }
    println!("{:?}", is_palindrome(args[1].clone()));
    Ok(())
}

fn is_palindrome(s: String) -> bool {
    // TODO: solve
    println!("{s:?}");
    false
}
