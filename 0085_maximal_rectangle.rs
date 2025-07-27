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
    let mut max_area = 0;
    for (r, row) in matrix.iter().enumerate() {
        let mut stack = Vec::with_capacity(row.len());
        for (c, _) in row.iter().enumerate() {
            let mut h = 0;
            for i in (0..=r).rev() {
                if matrix[i][c] == '1' {
                    h += 1;
                } else {
                    break;
                }
            }
            let mut start = c;
            while let Some((si, sh)) = stack.pop() {
                if sh <= h {
                    stack.push((si, sh));
                    break;
                }
                max_area = max_area.max(sh * ((c - si) as i32));
                start = si;
            }
            stack.push((start, h));
        }
        for (i, h) in stack.iter() {
            max_area = max_area.max(h * ((row.len() - i) as i32));
        }
    }
    max_area
}
