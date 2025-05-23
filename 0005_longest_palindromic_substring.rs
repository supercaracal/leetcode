fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: babad");
    }
    println!("{:?}", longest_palindrome(args[1].clone()));
    Ok(())
}

fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut substr = "";
    for i in 0..s.len() {
        for offset in 0..=1 {
            let mut l = i;
            let mut r = i + offset; // odd or even
            while r < s.len() && chars[l] == chars[r] {
                if r - l + 1 > substr.len() {
                    substr = &s.as_str()[l..=r];
                }
                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }
    }
    substr.to_string()
}
