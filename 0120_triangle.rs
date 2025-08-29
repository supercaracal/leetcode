fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err("usage: 2 3,4 6,5,7 4,1,8,3");
    }
    let mut triangle = Vec::new();
    for chunk in args.iter().skip(1) {
        let mut row = Vec::new();
        for e in chunk.split(',') {
            row.push(e.parse::<i32>().unwrap());
        }
        triangle.push(row);
    }
    println!("{:?}", minimum_total(triangle));
    Ok(())
}

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    // TODO: solve
    for row in triangle.iter() {
        println!("{row:?}");
    }
    0
}
