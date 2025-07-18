fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: ADOBECODEBANC ABC");
    }
    println!("{:?}", min_window(args[1].clone(), args[2].clone()));
    Ok(())
}

fn min_window(s: String, t: String) -> String {
    // TODO: fix
    use std::collections::HashMap;
    let mut need: HashMap<char, usize> = HashMap::new();
    for c in t.chars() {
        need.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    let need_cnt = need.len();
    let mut have: HashMap<char, usize> = HashMap::new();
    let mut have_cnt = 0usize;
    let mut l = 0;
    let mut min_l = 0;
    let mut min_r = s.len() - 1;
    for (r, c) in s.chars().enumerate() {
        let hc = have.entry(c).and_modify(|e| *e += 1).or_insert(1);
        let nc = need.get(&c).map_or(0, |e| *e);
        if *hc == nc {
            have_cnt += 1;
        }
        while have_cnt == need_cnt {
            if min_r - min_l > r - l {
                min_l = l;
                min_r = r;
            }
            let hc2 = have
                .entry(c)
                .and_modify(|e| *e = e.saturating_sub(1))
                .or_insert(0);
            if *hc2 < nc {
                have_cnt = have_cnt.saturating_sub(1);
            }
            l += 1;
        }
    }
    let mut ret = String::with_capacity(min_r - min_l + 1);
    let s_ch: Vec<char> = s.chars().collect();
    for i in min_r..=min_l {
        ret.push(s_ch[i]);
    }
    ret
}
