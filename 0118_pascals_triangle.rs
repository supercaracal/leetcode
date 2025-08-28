fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 5");
    }
    let num_rows = args[1].parse::<i32>().unwrap();
    for row in generate(num_rows).iter() {
        println!("{row:?}");
    }
    Ok(())
}

fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;
    let mut rows: Vec<Vec<i32>> = Vec::with_capacity(num_rows);
    for i in 1..=num_rows {
        let mut row = Vec::with_capacity(i);
        if i == 1 {
            row.push(1);
        } else {
            for j in 0..i {
                if j == 0 || j == i - 1 {
                    row.push(1);
                } else {
                    let v = rows[i - 2][j - 1] + rows[i - 2][j];
                    row.push(v);
                }
            }
        }
        rows.push(row);
    }
    rows
}
