fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", num_trees(n));
    Ok(())
}

fn num_trees(n: i32) -> i32 {
    use std::collections::HashMap;
    let mut cache = HashMap::new();
    fn count(l: usize, r: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if l > r {
            return 1;
        }
        if let Some(v) = cache.get(&(l, r)) {
            return *v;
        }
        let mut cnt = 0;
        for v in l..=r {
            cnt += count(l, v - 1, cache) * count(v + 1, r, cache);
        }
        cache.insert((l, r), cnt);
        cnt
    }
    count(1, n as usize, &mut cache) as i32
}
