fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: barfoothefoobarman foo,bar");
    }
    let words: Vec<String> = args[2].split(',').map(|e| e.to_string()).collect();
    println!("{:?}", find_substring(args[1].clone(), words));
    Ok(())
}

fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    // TODO: solve
    println!("{s:?}, {words:?}");
    vec![0]
}
