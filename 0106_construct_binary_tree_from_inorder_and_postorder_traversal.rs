use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn dig(node: &TreeNode, list: &mut std::fmt::DebugList) {
    list.entry(&node.val);
    if let Some(ref n) = node.left {
        dig(&n.clone().borrow(), list);
    }
    if let Some(ref n) = node.right {
        dig(&n.clone().borrow(), list);
    }
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        dig(self, &mut list);
        list.finish()
    }
}

fn parse_arg(arg: &str) -> Vec<i32> {
    arg.split(',').map(|e| e.parse::<i32>().unwrap()).collect()
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 9,3,15,20,7 9,15,7,20,3");
    }
    let inorder = parse_arg(args[1].as_str());
    let postorder = parse_arg(args[2].as_str());
    println!("{:?}", build_tree(inorder, postorder));
    Ok(())
}

fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::HashMap;
    let mut postorder = postorder;
    let mut cache = HashMap::new();
    for (i, v) in inorder.iter().enumerate() {
        cache.insert(*v, i as i32);
    }
    fn dfs(
        postorder: &mut Vec<i32>,
        l: i32,
        r: i32,
        cache: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }
        let rv = postorder.pop().unwrap();
        let root = Rc::new(RefCell::new(TreeNode::new(rv)));
        let m = cache.get(&rv).unwrap();
        let rr = root.clone();
        let mut rrc = rr.borrow_mut();
        rrc.right = dfs(postorder, m + 1, r, cache);
        rrc.left = dfs(postorder, l, m - 1, cache);
        Some(root.clone())
    }
    dfs(&mut postorder, 0, inorder.len() as i32 - 1, &cache)
}
