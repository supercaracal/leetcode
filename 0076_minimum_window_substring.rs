fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: ADOBECODEBANC ABC");
    }
    println!("{:?}", min_window(args[1].clone(), args[2].clone()));
    Ok(())
}

fn min_window(s: String, t: String) -> String {
    // TODO: solve
    println!("s={s:?}, t={t:?}");
    "".to_string()
}
