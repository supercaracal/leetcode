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

#[allow(clippy::comparison_chain)]
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut t = 0;
    let mut b = matrix.len() - 1;
    let mut x = 0;
    while t <= b {
        x = t + (b - t) / 2;
        let min = matrix[x][0];
        let max = matrix[x][matrix[x].len() - 1];
        if target >= min && target <= max {
            break;
        }
        if target > max {
            if x == matrix.len() - 1 {
                break;
            }
            t = x + 1;
            continue;
        }
        if target < min {
            if x == 0 {
                break;
            }
            b = x - 1;
        }
    }
    let mut r = 0;
    let mut l = matrix[x].len() - 1;
    while r <= l {
        let m = r + (l - r) / 2;
        if target > matrix[x][m] {
            if m == matrix[x].len() - 1 {
                break;
            }
            r = m + 1;
        } else if target < matrix[x][m] {
            if m == 0 {
                break;
            }
            l = m - 1;
        } else {
            return true;
        }
    }
    false
}
