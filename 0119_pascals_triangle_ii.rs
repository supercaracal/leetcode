fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let row_index = args[1].parse::<i32>().unwrap();
    println!("{:?}", get_row(row_index));
    Ok(())
}

fn get_row(row_index: i32) -> Vec<i32> {
    if row_index < 0 {
        return Vec::with_capacity(0);
    }
    let row_index = row_index as usize;
    let mut row = Vec::with_capacity(row_index + 1);
    for i in 0..=row_index {
        row.push(1);
        let mut prev = 1;
        for j in 0..=i {
            if j == 0 || j == i {
                continue;
            }
            let tmp = row[j];
            row[j] = prev + row[j];
            prev = tmp;
        }
    }
    row
}
