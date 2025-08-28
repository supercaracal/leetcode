fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let row_index = args[1].parse::<i32>().unwrap();
    println!("{:?}", get_row(row_index));
    Ok(())
}

fn get_row(row_index: i32) -> Vec<i32> {
    // TODO: solve
    println!("{row_index}");
    vec![]
}
