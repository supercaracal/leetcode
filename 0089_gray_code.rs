fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 2");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", gray_code(n));
    Ok(())
}

// https://www.youtube.com/watch?v=CHr3V8JDa1w
fn gray_code(n: i32) -> Vec<i32> {
    let size = 1 << n;
    let mut ret = Vec::with_capacity(size);
    for i in 0..size {
        let v = i ^ (i >> 1); // TODO: figure out
        ret.push(v as i32);
    }
    ret
}
