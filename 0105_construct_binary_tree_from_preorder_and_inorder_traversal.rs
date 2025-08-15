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
    if let Some(ref n) = node.left {
        dig(&n.clone().borrow(), list);
    }
    list.entry(&node.val);
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
        return Err("usage: 3,9,20,15,7 9,3,15,20,7");
    }
    let preorder = parse_arg(args[1].as_str());
    let inorder = parse_arg(args[2].as_str());
    println!("{:?}", build_tree(preorder, inorder));
    Ok(())
}

fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build(&preorder, &inorder)
}

fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    // https://www.youtube.com/watch?v=ihj4IQGZ2zc
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
    let mid = inorder.iter().position(|e| e == &preorder[0]).unwrap();
    let rr = root.clone();
    let mut rrc = rr.borrow_mut();
    rrc.left = build(&preorder[1..(mid + 1)], &inorder[..mid]);
    rrc.right = build(&preorder[(mid + 1)..], &inorder[(mid + 1)..]);
    Some(root.clone())
}
