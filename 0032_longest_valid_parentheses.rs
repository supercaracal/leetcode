fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: (()");
    }
    println!("{:?}", longest_valid_parentheses(args[1].clone()));
    Ok(())
}

pub fn longest_valid_parentheses(s: String) -> i32 {
    // FIXME: some errors
    if s.is_empty() {
        return 0;
    }
    let mut stack = Vec::with_capacity(s.len());
    let mut max = 0;
    for (r, c) in s.chars().enumerate() {
        match c {
            '(' => {
                stack.push(r);
            }
            ')' => if let Some(l) = stack.pop() {
                if stack.is_empty() {
                    max += r - l + 1;
                } else {
                    max = max.max(r - l + 1);
                }
            },
            _ => {}
        }
    }
    max as i32
}
