fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 'A man, a plan, a canal: Panama'");
    }
    println!("{:?}", is_palindrome(args[1].clone()));
    Ok(())
}

fn is_palindrome(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let mut l = 0;
    let mut r = s.len() - 1;
    while l <= r {
        while !s[l].is_ascii_alphanumeric() {
            if l + 1 == s.len() {
                break;
            }
            l += 1;
        }
        while !s[r].is_ascii_alphanumeric() {
            if r == 0 {
                break;
            }
            r -= 1;
        }
        println!("{}, {}", s[l], s[r]);
        if l + 1 == s.len() || r == 0 {
            break;
        } else if s[l].eq_ignore_ascii_case(&s[r]) {
            l += 1;
            r -= 1;
        } else {
            return false;
        }
    }
    true
}
