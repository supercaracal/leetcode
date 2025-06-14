fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 0,1,0,2,1,0,1,3,2,1,2,1");
    }
    let height: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", trap(height));
    Ok(())
}

fn trap(height: Vec<i32>) -> i32 {
    // TODO: solve
    println!("{height:?}");
    0
}
