fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 25525511135");
    }
    println!("{:?}", restore_ip_addresses(args[1].clone()));
    Ok(())
}

fn restore_ip_addresses(s: String) -> Vec<String> {
    // TODO: solve
    vec![s; 1]
}
