fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 23");
    }
    println!("{:?}", letter_combinations(args[1].clone()));
    Ok(())
}

use std::collections::HashMap;

fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let mut buttons = HashMap::new();
    buttons.insert(2, vec!['a', 'b', 'c']);
    buttons.insert(3, vec!['d', 'e', 'f']);
    buttons.insert(4, vec!['g', 'h', 'i']);
    buttons.insert(5, vec!['j', 'k', 'l']);
    buttons.insert(6, vec!['m', 'n', 'o']);
    buttons.insert(7, vec!['p', 'q', 'r', 's']);
    buttons.insert(8, vec!['t', 'u', 'v']);
    buttons.insert(9, vec!['w', 'x', 'y', 'z']);
    let digits: Vec<u32> = digits.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let size = digits
        .iter()
        .map(|d| buttons.get(d).unwrap().len())
        .reduce(|a, b| a * b)
        .unwrap();
    let mut combis = Vec::with_capacity(size);
    let mut s = String::with_capacity(digits.len());
    backtrack(0, &mut s, &digits, &mut combis, &buttons);
    combis
}

// https://www.algobreath.com/notes/letter-combinations-of-a-phone-number-in-rust
fn backtrack(
    i: usize,
    s: &mut String,
    digits: &Vec<u32>,
    combis: &mut Vec<String>,
    buttons: &HashMap<u32, Vec<char>>,
) {
    if s.len() == digits.len() {
        combis.push(s.clone());
        return;
    }
    if let Some(letters) = buttons.get(&digits[i]) {
        for c in letters.iter() {
            s.push(*c);
            backtrack(i + 1, s, digits, combis, buttons);
            let _ = s.pop(); // here!
        }
    }
}
