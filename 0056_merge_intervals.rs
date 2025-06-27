fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,3,2,6,8,10,15,18");
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
    println!("{:?}", merge(intervals));
    Ok(())
}

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: solve
    intervals
}
