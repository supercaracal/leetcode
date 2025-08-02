fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 12");
    }
    println!("{:?}", num_decodings(args[1].clone()));
    Ok(())
}

fn num_decodings(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<_>>();
    let mut cache = HashMap::new();
    backtrack(chars.as_slice(), 0, &mut cache)
}

use std::collections::HashMap;

fn backtrack(chars: &[char], i: usize, cache: &mut HashMap<usize, i32>) -> i32 {
    if let Some(v) = cache.get(&i) {
        return *v;
    }
    if i == chars.len() {
        return 1;
    }
    if chars[i] == '0' {
        return 0;
    }
    if i < chars.len() - 1 && chars[i] > '2' && chars[i + 1] == '0' {
        return 0;
    }
    let mut cnt = 0;
    cnt += backtrack(chars, i + 1, cache);
    if i == chars.len() - 1 {
        return cnt;
    }
    if i < chars.len() - 1 && (chars[i] > '2' || (chars[i] == '2' && chars[i + 1] > '6')) {
        return cnt;
    }
    cnt += backtrack(chars, i + 2, cache);
    cache.insert(i, cnt);
    cnt
}
