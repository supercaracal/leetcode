fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 0");
    }
    println!("{:?}", is_number(args[1].clone()));
    Ok(())
}

// https://www.youtube.com/watch?v=0qac3ngAZmE
fn is_number(s: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let mut sign = 0;
    let mut dot = false;
    let mut exponent = false;
    let mut digit = false;
    for i in 0..s.len() {
        match s[i] {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => digit = true,
            '-' | '+' => {
                if sign == 2 || (i > 0 && s[i - 1] != 'e' && s[i - 1] != 'E') || i == s.len() - 1 {
                    return false;
                }
                sign += 1;
            }
            '.' => {
                if dot || exponent || (!digit && i == s.len() - 1) {
                    return false;
                }
                dot = true
            }
            'e' | 'E' => {
                if exponent || !digit || i == s.len() - 1 {
                    return false;
                }
                exponent = true;
            }
            _ => return false,
        }
    }
    true
}
