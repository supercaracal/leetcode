fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: eat,tea,tan,ate,nat,bat");
    }
    let strs: Vec<String> = args[1].split(',').map(|v| v.to_string()).collect();
    println!("{:?}", group_anagrams(strs));
    Ok(())
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut dict: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut chars: Vec<_> = s.chars().collect();
        chars.sort();
        let mut k = String::with_capacity(chars.len());
        for c in chars {
            k.push(c);
        }
        dict.entry(k).or_default().push(s);
    }
    dict.into_values().collect()
}
