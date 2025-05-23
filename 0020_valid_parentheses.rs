fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: ()");
    }
    println!("{:?}", is_valid(args[1].clone()));
    Ok(())
}

fn is_valid(s: String) -> bool {
    let mut stack = Vec::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => match stack.pop() {
                Some(e) => {
                    if e != '(' {
                        return false;
                    }
                }
                None => return false,
            },
            ']' => match stack.pop() {
                Some(e) => {
                    if e != '[' {
                        return false;
                    }
                }
                None => return false,
            },
            '}' => match stack.pop() {
                Some(e) => {
                    if e != '{' {
                        return false;
                    }
                }
                None => return false,
            },
            _ => return false,
        }
    }
    stack.is_empty()
}
