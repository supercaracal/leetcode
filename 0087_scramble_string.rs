fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: great regeat");
    }
    println!("{:?}", is_scramble(args[1].clone(), args[2].clone()));
    Ok(())
}

fn is_scramble(s1: String, s2: String) -> bool {
    let mut cache = HashMap::new();
    dp(s1.as_str(), s2.as_str(), &mut cache)
}

use std::collections::HashMap;

// https://www.youtube.com/watch?v=MDmZm_aVDF8
fn dp<'a>(s1: &'a str, s2: &'a str, cache: &mut HashMap<(&'a str, &'a str), bool>) -> bool {
    let n = s1.len();
    if n == 1 {
        return s1 == s2;
    }
    if s1 == s2 {
        return true;
    }
    if let Some(ans) = cache.get(&(s1, s2)) {
        return *ans;
    }
    for i in 1..n {
        if (dp(&s1[0..i], &s2[0..i], cache) && dp(&s1[i..], &s2[i..], cache))
            || (dp(&s1[..i], &s2[(n - i)..], cache) && dp(&s1[i..], &s2[..(n - i)], cache))
        {
            cache.insert((s1, s2), true);
            return true;
        }
    }
    cache.insert((s1, s2), false);
    false
}
