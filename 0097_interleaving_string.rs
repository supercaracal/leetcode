fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: aabcc dbbca aadbbcbcac");
    }
    println!(
        "{:?}",
        is_interleave(args[1].clone(), args[2].clone(), args[3].clone())
    );
    Ok(())
}

fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    // TODO: solve
    println!("{s1:?} {s2:?} {s3:?}");
    false
}
