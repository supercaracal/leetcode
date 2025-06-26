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
    println!("{:?}", spiral_order(matrix));
    Ok(())
}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let size = m * n;
    let mut ret = Vec::with_capacity(size);
    let (mut t, mut b, mut l, mut r) = (0, m - 1, 0, n - 1);
    while ret.len() < size {
        for i in l..=r {
            ret.push(matrix[t][i]);
        }
        if ret.len() == size {
            break;
        }
        for i in (t + 1)..=b {
            ret.push(matrix[i][r]);
        }
        if ret.len() == size {
            break;
        }
        for i in (l..r).rev() {
            ret.push(matrix[b][i]);
        }
        if ret.len() == size {
            break;
        }
        for i in ((t + 1)..b).rev() {
            ret.push(matrix[i][l]);
        }
        t += 1;
        b -= 1;
        l += 1;
        r -= 1;
    }
    ret
}
