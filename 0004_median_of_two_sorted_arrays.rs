fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,2 3,4");
    }
    println!(
        "{:?}",
        find_median_sorted_arrays(parse_args(args[1].as_ref()), parse_args(args[2].as_ref()))
    );
    Ok(())
}

#[inline]
fn parse_args(arg: &str) -> Vec<i32> {
    arg.split(',').map(|e| e.parse::<i32>().unwrap()).collect()
}

// https://www.youtube.com/watch?v=q6IEA26hvXc
// Do binary search as is, Don't merge arrays naively
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (a, b) = if nums1.len() < nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };
    let (al, bl) = (a.len(), b.len());
    let mut left = 0;
    let mut right = al;
    let total = al + bl;
    let half = total.div_ceil(2);
    while left <= right {
        let i = (left + right) / 2;
        let j = half - i;
        let a_left_max = if i == 0 { i32::MIN } else { a[i - 1] };
        let a_right_min = if i == al { i32::MAX } else { a[i] };
        let b_left_max = if j == 0 { i32::MIN } else { b[j - 1] };
        let b_right_min = if j == bl { i32::MAX } else { b[j] };
        if a_left_max <= b_right_min && b_left_max <= a_right_min {
            if total % 2 == 0 {
                return (a_left_max.max(b_left_max) + a_right_min.min(b_right_min)) as f64 / 2.0;
            } else {
                return a_left_max.max(b_left_max) as _;
            }
        } else if a_left_max > b_right_min {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }
    0.0
}
