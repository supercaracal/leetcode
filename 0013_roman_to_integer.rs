fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: III");
    }
    println!("{:?}", roman_to_int(args[1].clone()));
    Ok(())
}

fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    let mut table = HashMap::new();
    table.insert('I', 1);
    table.insert('V', 5);
    table.insert('X', 10);
    table.insert('L', 50);
    table.insert('C', 100);
    table.insert('D', 500);
    table.insert('M', 1000);
    let mut num = 0;
    let mut prev = 0;
    for c in s.chars().rev() {
        let d = table.get(&c).unwrap();
        if *d >= prev {
            num += d;
        } else {
            num -= d;
        }
        prev = *d;
    }
    num
}
