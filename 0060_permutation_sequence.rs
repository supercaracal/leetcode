fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 3 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    let k = args[2].parse::<i32>().unwrap();
    println!("{:?}", get_permutation(n, k));
    Ok(())
}

fn get_permutation(n: i32, k: i32) -> String {
    let mut nums = (1..=n).collect();
    let mut perms = Vec::with_capacity((1..=n).fold(1, |a, e| a * e) as usize);
    let mut perm = Vec::with_capacity(n as usize);
    backtrack(n, &mut nums, &mut perm, &mut perms);
    let mut s = String::with_capacity(n as usize);
    for c in perms[(k - 1) as usize]
        .iter()
        .map(|e| char::from_digit(*e as u32, 10).unwrap())
    {
        s.push(c);
    }
    s
}

fn backtrack(n: i32, nums: &mut Vec<i32>, perm: &mut Vec<i32>, perms: &mut Vec<Vec<i32>>) {
    if perm.len() == (n as usize) {
        perms.push(perm.clone());
        return;
    }
    for i in 0..nums.len() {
        perm.push(nums.remove(i));
        backtrack(n, nums, perm, perms);
        nums.insert(i, perm.pop().unwrap());
    }
}
