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
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut ret = Vec::new();
    for i in 0..intervals.len() {
        let curr = &intervals[i];
        if ret.is_empty() {
            ret.push(curr.clone());
            continue;
        }
        let mut prev = ret.pop().unwrap();
        if prev[1] >= curr[0] && prev[1] >= curr[1] {
            ret.push(prev);
        } else if prev[1] >= curr[0] {
            prev[1] = curr[1];
            ret.push(prev);
        } else {
            ret.push(prev);
            ret.push(curr.clone());
        }
    }
    ret
}
