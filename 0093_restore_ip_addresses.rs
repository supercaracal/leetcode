fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 25525511135");
    }
    println!("{:?}", restore_ip_addresses(args[1].clone()));
    Ok(())
}

fn restore_ip_addresses(s: String) -> Vec<String> {
    let s = s.chars().collect::<Vec<_>>();
    let mut ip_addr = Vec::with_capacity(4);
    let mut ip_addrs = Vec::new();
    backtrack(&s, &mut ip_addr, &mut ip_addrs);
    ip_addrs
        .iter()
        .map(|v| {
            v.iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(".")
        })
        .collect()
}

fn backtrack(s: &[char], ip_addr: &mut Vec<u32>, ip_addrs: &mut Vec<Vec<u32>>) {
    if s.is_empty() || ip_addr.len() == 4 {
        if s.is_empty() && ip_addr.len() == 4 {
            ip_addrs.push(ip_addr.clone());
        }
        return;
    }
    ip_addr.push(s[0].to_digit(10).unwrap());
    backtrack(&s[1..], ip_addr, ip_addrs);
    ip_addr.pop();
    if s.len() >= 2 && s[0] != '0' {
        let d = s[0].to_digit(10).unwrap() * 10 + s[1].to_digit(10).unwrap();
        ip_addr.push(d);
        backtrack(&s[2..], ip_addr, ip_addrs);
        ip_addr.pop();
    }
    if s.len() >= 3 && s[0] != '0' {
        let d = s[0].to_digit(10).unwrap() * 100
            + s[1].to_digit(10).unwrap() * 10
            + s[2].to_digit(10).unwrap();
        if d < 256 {
            ip_addr.push(d);
            backtrack(&s[3..], ip_addr, ip_addrs);
            ip_addr.pop();
        }
    }
}
