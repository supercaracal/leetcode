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
        // TODO: fix
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
    if args.len() != 2 {
        return Err("usage: -10,9,20,null,null,15,7");
    }
    let root = build_tree(parse_arg(&args[1]));
    println!("{:?}", max_path_sum(root));
    Ok(())
}

// 1,2,null,3,null,4,null,5
//                 1
//           2           _
//       3       _
//    4     _
//  5
fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // TODO: fix
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(rr) = root {
            let rrc = rr.borrow();
            let (lv, lm) = helper(rrc.left.clone());
            let (rv, rm) = helper(rrc.right.clone());
            let max = (lv + rrc.val + rv)
                .max(lv + rrc.val)
                .max(rrc.val + rv)
                .max(rrc.val)
                .max(lm)
                .max(rm);
            return (rrc.val, max);
        }
        (0, i32::MIN)
    }
    helper(root).1
}
