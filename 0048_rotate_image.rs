fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,2,3,4,5,6,7,8,9 3");
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
    for row in matrix.iter() {
        println!("{row:?}");
    }
    println!("-----------------------");
    rotate(&mut matrix);
    for row in matrix.iter() {
        println!("{row:?}");
    }
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let size = matrix.len();
    let mut offset = 0;
    while offset < size / 2 {
        for i in 0..(size - offset * 2 - 1) {
            let mut tmp = matrix[0 + i + offset][size - 1 - offset];
            matrix[0 + i + offset][size - 1 - offset] = matrix[0 + offset][i + offset];
            matrix[0 + offset][i + offset] = tmp;

            tmp = matrix[size - 1 - offset][size - 1 - i - offset];
            matrix[size - 1 - offset][size - 1 - i - offset] = matrix[0 + offset][i + offset];
            matrix[0 + offset][i + offset] = tmp;

            tmp = matrix[size - 1 - i - offset][0 + offset];
            matrix[size - 1 - i - offset][0 + offset] = matrix[0 + offset][i + offset];
            matrix[0 + offset][i + offset] = tmp;
        }
        offset += 1;
    }
}
