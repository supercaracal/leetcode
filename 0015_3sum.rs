fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: -1,0,1,2,-1,-4");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", three_sum(nums));
    Ok(())
}

// https://www.youtube.com/watch?v=jzZsG8n2R9A
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;
    let mut nums = nums.to_owned();
    nums.sort();
    let mut triplets = vec![];
    for i in 0..nums.len() {
        if i > 0 && nums[i - 1] == nums[i] {
            continue;
        }
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                Ordering::Greater => r -= 1,
                Ordering::Less => l += 1,
                Ordering::Equal => {
                    triplets.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    while l < r && nums[l - 1] == nums[l] {
                        l += 1;
                    }
                }
            }
        }
    }
    triplets
}
