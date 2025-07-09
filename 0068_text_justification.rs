fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: This,is,an,example,of,text,justification. 16");
    }
    let words: Vec<String> = args[1].split(',').map(|e| e.to_string()).collect();
    let max_width = args[2].parse::<i32>().unwrap();
    println!("{:?}", full_justify(words, max_width));
    Ok(())
}

fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    // TODO: solve
    println!("{max_width}");
    words
}
