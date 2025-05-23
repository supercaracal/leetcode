fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3749");
    }
    let num = args[1].parse::<i32>().unwrap();
    println!("{:?}", int_to_roman(num));
    Ok(())
}

fn int_to_roman(num: i32) -> String {
    use std::collections::HashMap;
    let mut table = HashMap::new();
    table.insert(1, 'I');
    table.insert(5, 'V');
    table.insert(10, 'X');
    table.insert(50, 'L');
    table.insert(100, 'C');
    table.insert(500, 'D');
    table.insert(1000, 'M');
    let mut roman = "".to_string();
    let mut n = num;
    let mut d = 1;
    while n > 0 {
        let r = n % 10;
        println!("n={n:?}, r={r:?}, d={d:?}");
        match r {
            0 => {}
            x if x % 5 == 4 => {
                let s = table.get(&((x + 1) * d)).unwrap();
                roman.push(*s);
                let s = table.get(&d).unwrap();
                roman.push(*s);
            }
            _ => {
                let s = table.get(&d).unwrap();
                let mut i = r % 5;
                while i > 0 {
                    roman.push(*s);
                    i -= 1;
                }
                if r >= 5 {
                    if let Some(s) = table.get(&(5 * d)) {
                        roman.push(*s);
                    }
                }
            }
        }
        n /= 10;
        d *= 10;
    }
    roman.chars().rev().collect()
}
