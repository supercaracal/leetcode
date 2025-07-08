fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 11 1");
    }
    println!("{:?}", add_binary(args[1].clone(), args[2].clone()));
    Ok(())
}

fn add_binary(a: String, b: String) -> String {
    let mut a: Vec<char> = a.chars().collect();
    let mut b: Vec<char> = b.chars().collect();
    a.reverse();
    b.reverse();
    if b.len() > a.len() {
        let tmp = a;
        a = b;
        b = tmp;
    }
    for i in 0..a.len() {
        let sum = if i < b.len() {
            a[i].to_digit(10).unwrap() + b[i].to_digit(10).unwrap()
        } else {
            a[i].to_digit(10).unwrap()
        };
        if sum > 1 {
            let d = sum - 2;
            a[i] = char::from_digit(d, 10).unwrap();
            if i < a.len() - 1 {
                let c = a[i + 1].to_digit(10).unwrap() + 1;
                a[i + 1] = char::from_digit(c, 10).unwrap();
            } else {
                a.push('1');
            }
        } else {
            a[i] = char::from_digit(sum, 10).unwrap();
        }
    }
    String::from_iter(a.iter().rev())
}
