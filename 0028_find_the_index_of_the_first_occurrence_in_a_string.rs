fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: sadbutsad sad");
    }
    println!("{:?}", str_str(args[1].clone(), args[2].clone()));
    Ok(())
}

fn str_str(haystack: String, needle: String) -> i32 {
    println!("{haystack:?}, {needle:?}");
    0
}
