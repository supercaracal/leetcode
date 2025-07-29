fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: 1,2,3,0,0,0 3 2,5,6 3");
    }
    let mut nums1: Vec<i32> = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let m = args[2].parse::<i32>().unwrap();
    let mut nums2: Vec<i32> = args[3]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    let n = args[4].parse::<i32>().unwrap();
    merge(&mut nums1, m, &mut nums2, n);
    println!("{nums1:?}",);
    Ok(())
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    let mut p = nums1.len();
    while m > 0 || n > 0 {
        if m > 0 && n == 0 {
            nums1[p - 1] = nums1[m - 1];
            m -= 1;
        } else if m == 0 && n > 0 {
            nums1[p - 1] = nums2[n - 1];
            n -= 1;
        } else if nums1[m - 1] >= nums2[n - 1] {
            nums1[p - 1] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[p - 1] = nums2[n - 1];
            n -= 1;
        }
        p -= 1;
    }
}
