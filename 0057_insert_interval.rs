fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,3,6,9 2,5");
    }
    let mut intervals = Vec::new();
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    for pair in nums.chunks(2) {
        let mut interval = Vec::with_capacity(2);
        for n in pair {
            interval.push(*n);
        }
        intervals.push(interval);
    }
    let new_interval: Vec<i32> = args[2]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", insert(intervals, new_interval));
    Ok(())
}

fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    // TODO: solve
    println!("{intervals:?}");
    println!("{new_interval:?}");
    vec![]
}
