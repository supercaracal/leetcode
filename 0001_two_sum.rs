fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 2,7,11,15 9");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", two_sum(nums, target));
    Ok(())
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut dict: HashMap<i32, i32> = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        let r = target - n;
        if let Some(j) = dict.get(&r) {
            return vec![*j, i as i32];
        }
        dict.insert(*n, i as i32);
    }
    vec![]
}
