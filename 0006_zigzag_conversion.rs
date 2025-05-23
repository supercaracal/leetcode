fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: PAYPALISHIRING 3");
    }
    let num_rows = args[2].parse::<i32>().unwrap();
    println!("{:?}", convert(args[1].clone(), num_rows));
    Ok(())
}

fn convert(s: String, num_rows: i32) -> String {
    let n = num_rows as usize;
    let size = s.len();
    let step = (n * 2usize - 2usize).max(1);
    let chars: Vec<char> = s.chars().collect();
    let mut ret = "".to_string();
    for r in 0usize..n {
        for i in (r..s.len()).step_by(step) {
            ret.push(chars[i]);
            if r == 0usize || r == (n - 1usize) {
                continue;
            }
            let j = i + (n - r - 1usize) * 2usize;
            if j < size {
                ret.push(chars[j]); // zag
            }
        }
    }
    ret
}
