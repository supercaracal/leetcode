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

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: 3");
    }
    let n = args[1].parse::<i32>().unwrap();
    println!("{:?}", generate_trees(n));
    Ok(())
}

fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    backtrack(1, n as usize)
}

fn backtrack(l: usize, r: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    // https://www.youtube.com/watch?v=m907FlQa2Yc
    if l > r {
        return vec![None; 1];
    }
    let mut trees = Vec::new();
    for v in l..=r {
        for left in backtrack(l, v - 1).iter() {
            for right in backtrack(v + 1, r).iter() {
                let mut node = TreeNode::new(v as i32);
                node.left = left.clone();
                node.right = right.clone();
                let root = Some(Rc::new(RefCell::new(node)));
                trees.push(root);
            }
        }
    }
    trees
}
