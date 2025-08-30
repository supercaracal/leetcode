fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err("usage: 2 3,4 6,5,7 4,1,8,3");
    }
    let mut triangle = Vec::new();
    for chunk in args.iter().skip(1) {
        let mut row = Vec::new();
        for e in chunk.split(',') {
            row.push(e.parse::<i32>().unwrap());
        }
        triangle.push(row);
    }
    println!("{:?}", minimum_total(triangle));
    Ok(())
}

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    fn backtrack(
        triangle: &Vec<Vec<i32>>,
        i: usize,
        j: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(v) = cache.get(&(i, j)) {
            return *v;
        }
        let v = if i == triangle.len() || j == triangle[i].len() {
            0
        } else {
            let l = triangle[i][j] + backtrack(triangle, i + 1, j, cache);
            if j + 1 == triangle[i].len() {
                l
            } else {
                let r = triangle[i][j + 1] + backtrack(triangle, i + 1, j + 1, cache);
                l.min(r)
            }
        };
        cache.insert((i, j), v);
        v
    }
    let mut cache = HashMap::new();
    backtrack(&triangle, 0, 0, &mut cache)
}
