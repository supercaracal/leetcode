fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: This,is,an,example,of,text,justification. 16");
    }
    let words: Vec<String> = args[1].split(',').map(|e| e.to_string()).collect();
    let max_width = args[2].parse::<i32>().unwrap();
    for line in full_justify(words, max_width) {
        println!("{line:?}");
    }
    Ok(())
}

fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut ret = Vec::new();
    let max_width = max_width as usize;
    let mut start = 0;
    let mut width = 0;
    let mut count = 0;
    for i in 0..words.len() {
        let min_sp_cnt = if count == 0 { 0 } else { count - 1 };
        if max_width <= width + min_sp_cnt + words[i].len() && start < i {
            let line = justify(max_width, width, count, start..i, &words, false);
            ret.push(line);
            start = i;
            width = 0;
            count = 0;
        }
        width += words[i].len();
        count += 1;
        if i == words.len() - 1 {
            let line = justify(max_width, width, count, start..words.len(), &words, true);
            ret.push(line);
        }
    }
    ret
}

fn justify(
    max_width: usize,
    width: usize,
    count: usize,
    indices: std::ops::Range<usize>,
    words: &[String],
    is_last: bool,
) -> String {
    let mut line = String::with_capacity(max_width);
    let mut total_sp_cnt = max_width - width;
    let sp_cnt = total_sp_cnt / (count.max(2) - 1);
    let mut sp_cnt_r = total_sp_cnt % (count.max(2) - 1);
    for i in indices {
        line.push_str(words[i].as_str());
        if max_width == line.len() {
            continue;
        }
        if is_last {
            line.push(' ');
            continue;
        }
        if total_sp_cnt < 1 {
            continue;
        }
        for _ in 0..sp_cnt {
            line.push(' ');
        }
        if sp_cnt_r > 0 {
            line.push(' ');
            sp_cnt_r -= 1;
            total_sp_cnt -= sp_cnt;
        }
        if total_sp_cnt >= sp_cnt {
            total_sp_cnt -= sp_cnt;
        }
    }
    if max_width > line.len() {
        for _ in 0..(max_width - line.len()) {
            line.push(' ');
        }
    }
    line
}
