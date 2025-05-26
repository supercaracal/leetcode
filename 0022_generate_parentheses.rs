fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", generate_parenthesis(n));
    Ok(())
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut ret = vec![];
    let size = (n * 2) as usize;
    let mut s = String::with_capacity(size);
    backtrack(0, &mut s, size, &mut ret);
    ret
}

fn backtrack(stack: usize, s: &mut String, size: usize, ret: &mut Vec<String>) {
    if s.len() == size {
        ret.push(s.clone());
        return;
    }
    if (size - s.len()) > stack {
        s.push('(');
        backtrack(stack + 1, s, size, ret);
        s.pop();
        if stack > 0 {
            s.push(')');
            backtrack(stack - 1, s, size, ret);
            s.pop();
        }
    } else {
        s.push(')');
        backtrack(stack - 1, s, size, ret);
        s.pop();
    }
}
