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
    if args.len() != 2 {
        return Err("usage: 2,1,3");
    }
    let mut root = build_tree(parse_arg(&args[1]));
    recover_tree(&mut root);
    println!("{root:?}");
    Ok(())
}

fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut list = Vec::new();
    bfs(root, &mut list);
    let mut a = None;
    let mut b = None;
    for i in 0..list.len() {
        if i + 1 < list.len() && list[i] > list[i + 1] {
            if a.is_none() {
                a = Some(list[i]);
            }
            b = Some(list[i + 1]);
        }
    }
    match (a, b) {
        (Some(a), Some(b)) => {
            recover(root, a, b);
        }
        _ => {}
    }
}

fn bfs(head: &mut Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
    if let Some(h) = head {
        let mut n = h.borrow_mut();
        bfs(&mut n.left, list);
        list.push(n.val);
        bfs(&mut n.right, list);
    }
}

fn recover(head: &mut Option<Rc<RefCell<TreeNode>>>, a: i32, b: i32) {
    if let Some(h) = head {
        let mut n = h.borrow_mut();
        if n.val == a {
            n.val = b;
        } else if n.val == b {
            n.val = a;
        }
        recover(&mut n.left, a, b);
        recover(&mut n.right, a, b);
    }
}
