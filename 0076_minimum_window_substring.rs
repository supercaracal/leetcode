fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: ADOBECODEBANC ABC");
    }
    println!("{:?}", min_window(args[1].clone(), args[2].clone()));
    Ok(())
}

fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;
    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        let indices = map.entry(c).or_default();
        indices.push(i);
    }
    let mut targets = Vec::with_capacity(t.len());
    let mut seen: HashMap<char, usize> = HashMap::new();
    for c in t.chars() {
        match map.get(&c) {
            Some(indices) => targets.push(indices),
            None => return "".to_string(),
        }
        seen.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    for (k, v) in seen.iter() {
        if let Some(indices) = map.get(k) {
            if *v > indices.len() {
                return "".to_string();
            }
        }
    }
    let mut perm = Vec::with_capacity(t.len());
    let mut perms = Vec::new();
    permutation(0, &targets, &mut perm, &mut perms);
    let idx = perms
        .iter()
        .min_by(|x, y| {
            let x_diff = x.iter().max().unwrap() - x.iter().min().unwrap();
            let y_diff = y.iter().max().unwrap() - y.iter().min().unwrap();
            x_diff.cmp(&y_diff)
        })
        .unwrap();
    let s_chars: Vec<char> = s.chars().collect();
    let mut ret = "".to_string();
    let r = idx.iter().min().unwrap().clone() as usize;
    let l = idx.iter().max().unwrap().clone() as usize;
    for i in r..=l {
        ret.push(s_chars[i]);
    }
    ret
}

fn permutation(
    i: usize,
    targets: &Vec<&Vec<usize>>,
    perm: &mut Vec<usize>,
    perms: &mut Vec<Vec<usize>>,
) {
    if perm.len() == targets.len() {
        perms.push(perm.clone());
        return;
    }
    for j in 0..targets[i].len() {
        perm.push(targets[i][j]);
        permutation(i + 1, targets, perm, perms);
        perm.pop();
    }
}
