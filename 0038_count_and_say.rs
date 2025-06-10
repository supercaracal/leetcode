fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 4");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", count_and_say(n));
    Ok(())
}

fn count_and_say(n: i32) -> String {
    if n == 0 {
        return "".to_string();
    }
    if n == 1 {
        return "1".to_string();
    }
    let mut encoded: Vec<(char, usize)> = Vec::new();
    for c in count_and_say(n - 1).chars() {
        if let Some(mut e) = encoded.pop() {
            if c == e.0 {
                e.1 += 1;
                encoded.push(e);
            } else {
                encoded.push(e);
                encoded.push((c, 1));
            }
        } else {
            encoded.push((c, 1));
        }
    }
    encoded.iter().fold("".to_string(), |mut acc, e| {
        acc.push_str(e.1.to_string().as_str());
        acc.push(e.0);
        acc
    })
}
