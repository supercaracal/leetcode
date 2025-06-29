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

// https://www.youtube.com/watch?v=A8NUOmlwOlM
fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = Vec::new();
    let mut new_interval = new_interval;
    for i in 0..intervals.len() {
        if new_interval[1] < intervals[i][0] {
            ret.push(new_interval.clone());
            for j in i..intervals.len() {
                ret.push(intervals[j].clone());
            }
            return ret;
        }
        if new_interval[0] > intervals[i][1] {
            ret.push(intervals[i].clone());
            continue;
        }
        new_interval = vec![
            intervals[i][0].min(new_interval[0]),
            intervals[i][1].max(new_interval[1]),
        ];
    }
    ret.push(new_interval);
    ret
}
