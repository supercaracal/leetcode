fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: sadbutsad sad");
    }
    println!("{:?}", str_str(args[1].clone(), args[2].clone()));
    Ok(())
}

fn str_str(haystack: String, needle: String) -> i32 {
    let haystack: Vec<char> = haystack.chars().collect();
    let needle: Vec<char> = needle.chars().collect();
    for i in 0..haystack.len() {
        for j in 0..needle.len() {
            if i + j >= haystack.len() {
                return -1;
            }
            if haystack[i + j] != needle[j] {
                break;
            }
            if j == needle.len() - 1 {
                return i as i32;
            }
        }
    }
    -1
}
