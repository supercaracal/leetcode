fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 123");
    }
    let x = args[1].parse::<i32>().unwrap();
    println!("{:?}", reverse(x));
    Ok(())
}

fn reverse(x: i32) -> i32 {
    let mut s = "".to_string();
    if x < 0 {
        s.push('-');
    }
    let mut prefix = true;
    for c in x.to_string().chars().rev() {
        if c != '0' {
            prefix = false;
        }
        if prefix && c == '0' {
            continue;
        }
        if c == '-' {
            continue;
        }
        s.push(c);
    }
    s.parse::<i32>().map_or(0, |e| e)
}
