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

fn parse_arg(arg: &str) -> Vec<Option<i32>> {
    arg.split(',').map(|e| e.parse::<i32>().ok()).collect()
}

//           0
//      1          2
//   3    4     5     6
//  7 8  9 10 11 12 13 14
fn build_tree(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let nodes = nums
        .iter()
        .map(|v| v.and_then(|e| Some(Rc::new(RefCell::new(TreeNode::new(e))))))
        .collect::<Vec<_>>();
    let mut offset = 1;
    for i in 0..nodes.len() {
        if let Some(node) = nodes[i].clone() {
            if i + offset < nodes.len() {
                node.borrow_mut().left = nodes[i + offset].clone();
            }
            offset += 1;
            if i + offset < nodes.len() {
                node.borrow_mut().right = nodes[i + offset].clone();
            }
        }
    }
    nodes[0].clone()
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 1,2,3 1,2,3");
    }
    let p = build_tree(parse_arg(&args[1]));
    let q = build_tree(parse_arg(&args[2]));
    println!("{}", is_same_tree(p, q));
    Ok(())
}

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(pr), Some(qr)) => {
            if pr.borrow().val != qr.borrow().val {
                false
            } else if !is_same_tree(pr.borrow().left.clone(), qr.borrow().left.clone()) {
                false
            } else if !is_same_tree(pr.borrow().right.clone(), qr.borrow().right.clone()) {
                false
            } else {
                true
            }
        }
        (Some(_), None) => false,
        (None, Some(_)) => false,
        _ => true,
    }
}
