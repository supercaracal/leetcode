fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 12");
    }
    println!("{:?}", num_decodings(args[1].clone()));
    Ok(())
}

fn num_decodings(s: String) -> i32 {
    let mut msg = Vec::new();
    let mut msgs = Vec::new();
    let chars = s.chars().collect::<Vec<_>>();
    backtrack(chars.as_slice(), 0, &mut msg, &mut msgs);
    println!("{msgs:?}");
    msgs.len() as i32
}

fn backtrack(chars: &[char], i: usize, msg: &mut Vec<u8>, msgs: &mut Vec<Vec<u8>>) {
    if i == chars.len() {
        msgs.push(msg.clone());
        return;
    }
    if chars[i] == '0' {
        return;
    }
    if i < chars.len() - 1 && chars[i] > '2' && chars[i + 1] == '0' {
        return;
    }
    msg.push(chars[i].to_digit(10).map_or(0, |v| v) as u8);
    backtrack(chars, i + 1, msg, msgs);
    msg.pop();
    if i == chars.len() - 1 {
        return;
    }
    if i < chars.len() - 1 && (chars[i] > '2' || (chars[i] == '2' && chars[i + 1] > '6')) {
        return;
    }
    let d10 = chars[i].to_digit(10).map_or(0, |v| v * 10);
    let d1 = chars[i + 1].to_digit(10).map_or(0, |v| v);
    msg.push((d10 + d1) as u8);
    backtrack(chars, i + 2, msg, msgs);
    msg.pop();
}
