fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: flower,flow,flight");
    }
    let strs = args[1].split(',').map(|e| e.to_string()).collect();
    println!("{:?}", longest_common_prefix(strs));
    Ok(())
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut i = 0;
    let chars: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
    let max_len = strs.iter().map(|s| s.len()).max().map_or(0, |e| e);
    while i < max_len {
        let mut j = 0;
        let mut c = '\0';
        while j < strs.len() {
            if i == strs[j].len() {
                return strs[0][0..i].to_string();
            }
            if c != '\0' && c != chars[j][i] {
                return strs[0][0..i].to_string();
            }
            c = chars[j][i];
            j += 1;
        }
        i += 1;
    }
    strs[0][0..i].to_string()
}
