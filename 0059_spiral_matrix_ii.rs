fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", generate_matrix(n));
    Ok(())
}

#[allow(clippy::needless_range_loop)]
fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let (mut t, mut b, mut l, mut r) = (0, n as usize - 1, 0, n as usize - 1);
    let mut matrix = vec![vec![0; n as usize]; n as usize];
    let mut number = 1i32;
    while l <= r && t <= b {
        for i in l..=r {
            matrix[t][i] = number;
            number += 1;
        }
        t += 1;
        for i in t..=b {
            matrix[i][r] = number;
            number += 1;
        }
        if r == 0 {
            break;
        }
        r -= 1;
        for i in (l..=r).rev() {
            matrix[b][i] = number;
            number += 1;
        }
        if b == 0 {
            break;
        }
        b -= 1;
        for i in (t..=b).rev() {
            matrix[i][l] = number;
            number += 1;
        }
        l += 1;
    }
    matrix
}
