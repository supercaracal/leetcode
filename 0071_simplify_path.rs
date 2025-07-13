fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: /home/");
    }
    println!("{:?}", simplify_path(args[1].clone()));
    Ok(())
}

fn simplify_path(path: String) -> String {
    let mut layers: Vec<String> = Vec::new();
    let mut layer = String::new();
    let mut path = path;
    path.push('/');
    for c in path.chars() {
        match c {
            '/' => {
                if layer.is_empty() {
                    // root
                } else if layer == "." {
                    // current
                } else if layer == ".." {
                    layers.pop();
                } else {
                    layers.push(layer.clone());
                }
                layer.clear();
            }
            _ => layer.push(c),
        }
    }
    let mut ret = "/".to_string();
    ret.push_str(layers.join("/").as_str());
    ret
}
