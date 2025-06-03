fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: barfoothefoobarman foo,bar");
    }
    let words: Vec<String> = args[2].split(',').map(|e| e.to_string()).collect();
    println!("{:?}", find_substring(args[1].clone(), words));
    Ok(())
}

fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    if s.is_empty() || words.is_empty() {
        return Vec::with_capacity(0);
    }
    let mut dict = std::collections::HashMap::new();
    for word in words.iter() {
        dict.insert(word.as_str(), ());
    }
    let size = words[0].len();
    let mut indices = Vec::new();
    for i in 0..s.len() {
        let mut j = i;
        let mut uniq = std::collections::HashMap::new();
        while j + size <= s.len() && j + size <= i + size * words.len() {
            let substr: &str = &s[j..j + size];
            if uniq.contains_key(substr) {
                break;
            }
            if dict.get(substr).is_some() {
                uniq.insert(substr, ());
                j += size;
            } else {
                break;
            }
        }
        if i + size * words.len() <= j {
            indices.push(i as i32);
        }
    }
    indices
}
