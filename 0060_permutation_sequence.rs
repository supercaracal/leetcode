fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 3 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    let k = args[2].parse::<i32>().unwrap();
    println!("{:?}", get_permutation(n, k));
    Ok(())
}

fn get_permutation(n: i32, k: i32) -> String {
    println!("n={n}, k={k}");
    "TODO: solve".to_string()
}
