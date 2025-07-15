fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: 1,3,5,7,10,11,16,20,23,30,34,60 4 3");
    }
    let size = args[2].parse::<usize>().unwrap();
    let mut matrix = Vec::with_capacity(size);
    for nums in args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .chunks(size)
    {
        let mut row = Vec::with_capacity(size);
        for n in nums {
            row.push(*n);
        }
        matrix.push(row);
    }
    let target = args[3].parse::<i32>().unwrap();
    println!("{}", search_matrix(matrix, target));
    Ok(())
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // TODO: solve
    for row in matrix.iter() {
        println!("{row:?}");
    }
    println!("{target}");
    false
}
