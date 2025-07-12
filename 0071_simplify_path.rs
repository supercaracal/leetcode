fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: /home/");
    }
    println!("{:?}", simplify_path(args[1].clone()));
    Ok(())
}

fn simplify_path(path: String) -> String {
    // TODO: solve
    path
}
