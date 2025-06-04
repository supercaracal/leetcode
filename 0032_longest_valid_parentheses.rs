fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: (()");
    }
    println!("{:?}", longest_valid_parentheses(args[1].clone()));
    Ok(())
}

pub fn longest_valid_parentheses(s: String) -> i32 {
    s.len() as i32
}
