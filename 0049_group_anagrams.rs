fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: eat,tea,tan,ate,nat,bat");
    }
    let strs: Vec<String> = args[1].split(',').map(|v| v.to_string()).collect();
    println!("{:?}", group_anagrams(strs));
    Ok(())
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // TODO: solve
    println!("{strs:?}");
    vec![vec![]]
}
