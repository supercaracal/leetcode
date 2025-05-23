fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: abcabcbb");
    }
    println!("{:?}", length_of_longest_substring(args[1].clone()));
    Ok(())
}

fn length_of_longest_substring(s: String) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;
    let mut dict: HashMap<char, usize> = HashMap::new();
    let mut max_size = 0;
    if !s.is_empty() {
        max_size = 1;
    }
    let mut start = 0;
    for (i, c) in s.chars().enumerate() {
        if let Some(j) = dict.get(&c) {
            start = max(start, *j + 1);
        }
        max_size = max(max_size, i - start + 1);
        dict.insert(c, i);
    }
    max_size as i32
}
