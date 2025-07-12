fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", climb_stairs(n));
    Ok(())
}

fn climb_stairs(n: i32) -> i32 {
    let mut cache = HashMap::new();
    // TODO: optimize, N(k) = N(k - 1) + N(k - 2)
    backtrack(n, &mut cache)
}

use std::collections::HashMap;

fn backtrack(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(cnt) = cache.get(&n) {
        return *cnt;
    }
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }
    let mut cnt = 0;
    for i in 1..=2 {
        cnt += backtrack(n - i, cache);
    }
    cache.insert(n, cnt);
    cnt
}
