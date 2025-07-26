fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2,1,5,6,2,3");
    }
    let heights: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", largest_rectangle_area(heights));
    Ok(())
}

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    // https://www.youtube.com/watch?v=zx5Sw9130L0
    let mut max_area = 0;
    let mut stack = Vec::with_capacity(heights.len());
    for (i, h) in heights.iter().enumerate() {
        let mut start = i;
        while let Some((si, sh)) = stack.pop() {
            if sh <= *h {
                stack.push((si, sh));
                break;
            }
            max_area = max_area.max(sh * ((i - si) as i32));
            start = si;
        }
        stack.push((start, *h));
    }
    for (i, h) in stack.iter() {
        max_area = max_area.max(h * ((heights.len() - i) as i32));
    }
    max_area
}
