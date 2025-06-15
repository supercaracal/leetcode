fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 0,1,0,2,1,0,1,3,2,1,2,1");
    }
    let height: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", trap(height));
    Ok(())
}

fn trap(height: Vec<i32>) -> i32 {
    let len = height.len();
    let mut l = 0;
    let mut r = len - 1;
    let mut max_l = 0;
    let mut max_r = 0;
    let mut area = 0;
    while l < r {
        max_l = max_l.max(height[l]);
        max_r = max_r.max(height[r]);
        if height[l] <= height[r] {
            let n = max_l - height[l];
            if n > 0 {
                area += n;
            }
            l += 1;
        } else {
            let n = max_r - height[r];
            if n > 0 {
                area += n;
            }
            r -= 1;
        }
    }
    area
}
