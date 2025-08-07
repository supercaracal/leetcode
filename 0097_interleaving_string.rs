fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: aabcc dbbca aadbbcbcac");
    }
    println!(
        "{:?}",
        is_interleave(args[1].clone(), args[2].clone(), args[3].clone())
    );
    Ok(())
}

fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let mut cache = HashMap::new();
    backtrack(s1.as_str(), s2.as_str(), s3.as_str(), &mut cache)
}

use std::collections::HashMap;

fn backtrack(s1: &str, s2: &str, s3: &str, cache: &mut HashMap<(usize, usize), bool>) -> bool {
    if s1.is_empty() && s2.is_empty() && s3.is_empty() {
        return true;
    }
    if let Some(v) = cache.get(&(s1.len(), s2.len())) {
        return *v;
    }
    if s1.len() > 0 && s3.len() > 0 && s1[0..1] == s3[0..1] {
        let v = backtrack(s1[1..].as_ref(), s2, s3[1..].as_ref(), cache);
        cache.insert((s1.len() - 1, s2.len()), v);
        if v {
            return true;
        }
    }
    if s2.len() > 0 && s3.len() > 0 && s2[0..1] == s3[0..1] {
        let v = backtrack(s1, s2[1..].as_ref(), s3[1..].as_ref(), cache);
        cache.insert((s1.len(), s2.len() - 1), v);
        if v {
            return true;
        }
    }
    false
}
