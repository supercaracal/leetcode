fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: hit cog hot,dot,dog,lot,log,cog");
    }
    let begin_word = args[1].clone();
    let end_word = args[2].clone();
    let word_list: Vec<String> = args[3].split(',').map(|e| e.to_string()).collect();
    println!("{:?}", find_ladders(begin_word, end_word, word_list));
    Ok(())
}

fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    // TODO: solve
    println!("{begin_word:?}");
    println!("{end_word:?}");
    println!("{word_list:?}");
    vec![]
}
