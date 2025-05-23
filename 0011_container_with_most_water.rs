fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 1,8,6,2,5,4,8,3,7");
    }
    let height: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", max_area(height));
    Ok(())
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;
    let mut max = 0;
    while i < j {
        let d = (j - i) as i32;
        let area = height[i].min(height[j]) * d;
        max = max.max(area);
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    max
}
