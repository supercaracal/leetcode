fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,3,5,6 5");
    }
    let nums: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let target = args[2].parse::<i32>().unwrap();
    println!("{:?}", search_insert(nums, target));
    Ok(())
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    use std::cmp::Ordering;
    while l <= r {
        let m = (l + r) / 2;
        match target.cmp(&nums[m]) {
            Ordering::Greater => {
                if m == nums.len() - 1 {
                    l = m + 1;
                    break;
                }
                l = m + 1;
            }
            Ordering::Less => {
                if m == 0 {
                    break;
                }
                r = m - 1;
            }
            Ordering::Equal => {
                l = m;
                break;
            }
        }
    }
    l as i32
}
