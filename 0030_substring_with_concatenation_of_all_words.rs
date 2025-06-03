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
        let cnt = if let Some(v) = dict.get(word.as_str()) {
            *v + 1
        } else {
            1
        };
        dict.insert(word.as_str(), cnt);
    }
    let size = words[0].len();
    let mut indices = Vec::new();
    for i in 0..s.len() {
        let mut j = i;
        let mut uniq = std::collections::HashMap::new();
        while j + size <= s.len() && j + size <= i + size * words.len() {
            let substr: &str = &s[j..j + size];
            if let Some(max) = dict.get(substr) {
                let cnt = if let Some(v) = uniq.get(substr) {
                    if *v == *max {
                        break;
                    }
                    *v + 1
                } else {
                    1
                };
                uniq.insert(substr, cnt);
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
