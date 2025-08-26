fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: rabbbit rabbit");
    }
    println!("{:?}", num_distinct(args[1].clone(), args[2].clone()));
    Ok(())
}

fn num_distinct(s: String, t: String) -> i32 {
    use std::collections::HashMap;
    fn backtrack(
        s: &[char],
        t: &[char],
        si: usize,
        ti: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if ti == t.len() {
            return 1;
        }
        if si == s.len() {
            return 0;
        }
        if let Some(v) = cache.get(&(si, ti)) {
            return *v;
        }
        let mut count = 0;
        if s[si] == t[ti] {
            count += backtrack(s, t, si + 1, ti + 1, cache);
        }
        count += backtrack(s, t, si + 1, ti, cache);
        cache.insert((si, ti), count);
        count
    }
    if s.is_empty() || t.is_empty() {
        return 0;
    }
    let sc = s.chars().collect::<Vec<char>>();
    let tc = t.chars().collect::<Vec<char>>();
    let mut cache = HashMap::new();
    backtrack(&sc, &tc, 0, 0, &mut cache)
}
