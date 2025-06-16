fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: aa a");
    }
    println!("{:?}", is_match(args[1].clone(), args[2].clone()));
    Ok(())
}

fn is_match(s: String, p: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let mut cache: Vec<Vec<Option<bool>>> = Vec::with_capacity(s.len());
    for _ in 0..s.len() {
        let row = vec![None; p.len()];
        cache.push(row);
    }
    dfs(&s, &p, 0, 0, &mut cache)
}

fn dfs(s: &[char], p: &[char], i: usize, j: usize, cache: &mut Vec<Vec<Option<bool>>>) -> bool {
    if i >= s.len() && j >= p.len() {
        return true;
    }
    if j >= p.len() {
        return false;
    }
    if i < s.len() && j < p.len() && cache[i][j].is_some() {
        return cache[i][j].unwrap();
    }
    let matched = i < s.len() && (s[i] == p[j] || p[j] == '?' || p[j] == '*');
    if p[j] == '*' {
        let result = dfs(s, p, i, j + 1, cache) || (matched && dfs(s, p, i + 1, j, cache));
        if i < s.len() && j < p.len() {
            cache[i][j] = Some(result);
        }
        return result;
    }
    if matched {
        let result = dfs(s, p, i + 1, j + 1, cache);
        if i < s.len() && j < p.len() {
            cache[i][j] = Some(result);
        }
        return result;
    }
    if i < s.len() && j < p.len() {
        cache[i][j] = Some(false);
    }
    false
}
