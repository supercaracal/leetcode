fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,0,1,0,0,1,0,1,1,1,1,1,1,1,1,1,0,0,1,0 5");
    }
    let size = args[2].parse::<usize>().unwrap();
    let mut matrix = Vec::new();
    for row in args[1]
        .chars()
        .filter(|c| *c != ',')
        .collect::<Vec<char>>()
        .chunks(size)
    {
        matrix.push(row.to_vec());
    }
    println!("{}", maximal_rectangle(matrix));
    Ok(())
}

fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    // TODO: solve
    for row in matrix.iter() {
        println!("{row:?}");
    }
    0
}
