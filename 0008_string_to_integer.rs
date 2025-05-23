fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 42");
    }
    println!("{:?}", my_atoi(args[1].clone()));
    Ok(())
}

fn my_atoi(s: String) -> i32 {
    let mut ignorable = true;
    let mut negative = false;
    let mut ret = 0;
    let (min, max) = (i32::MIN / 10, i32::MAX / 10);
    for c in s.chars() {
        match c {
            ' ' | '+' | '-' => {
                if ignorable {
                    if c == '-' {
                        negative = true;
                    }
                    if c != ' ' {
                        ignorable = false;
                    }
                    continue;
                } else {
                    break;
                }
            }
            n if n.is_ascii_digit() => {
                if ignorable {
                    ignorable = false;
                }
                let d = n.to_digit(10).unwrap() as i32;
                if negative {
                    if ret < min || i32::MIN + d > ret * 10 {
                        return i32::MIN;
                    }
                    ret = ret * 10 - d;
                } else {
                    if ret > max || i32::MAX - d < ret * 10 {
                        return i32::MAX;
                    }
                    ret = ret * 10 + d;
                }
            }
            _ => break,
        }
    }
    ret
}
