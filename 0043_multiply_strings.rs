fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 2 3");
    }
    println!("{:?}", multiply(args[1].clone(), args[2].clone()));
    Ok(())
}

fn multiply(num1: String, num2: String) -> String {
    // OPTIMIZE: tweak
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }
    let mut sums = Vec::new();
    let mut max = 0;
    for (i, n1) in num1.chars().rev().enumerate() {
        for (j, n2) in num2.chars().rev().enumerate() {
            let p = n1.to_digit(10).unwrap();
            let q = n2.to_digit(10).unwrap();
            let mut sum = vec![0; i + j];
            let mut v = p * q;
            while v > 0 {
                sum.push(v % 10);
                v /= 10;
            }
            max = max.max(sum.len());
            sums.push(sum);
        }
    }
    let mut ret = Vec::new();
    let mut d = 0;
    for i in 0..max {
        for sum in sums.iter() {
            if i < sum.len() {
                d += sum[i];
            }
        }
        ret.push(char::from_digit(d % 10, 10).unwrap());
        d /= 10;
    }
    while d > 0 {
        ret.push(char::from_digit(d % 10, 10).unwrap());
        d /= 10;
    }
    ret.reverse();
    ret.into_iter().collect()
}
