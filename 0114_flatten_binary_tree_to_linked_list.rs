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
        return Err("usage: 1,2,5,3,4,null,6");
    }
    let mut root = build_tree(parse_arg(&args[1]));
    flatten(&mut root);
    println!("{root:?}");
    Ok(())
}

fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn concat(
        root: Option<Rc<RefCell<TreeNode>>>,
        leaf: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(rr) = root {
            let mut rrc = rr.borrow_mut();
            rrc.right = concat(rrc.right.take(), leaf);
            Some(rr.clone())
        } else {
            leaf
        }
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(rr) = root {
            let mut rrc = rr.borrow_mut();
            match (dfs(rrc.left.take()), dfs(rrc.right.take())) {
                (Some(lr), Some(rr)) => rrc.right = concat(Some(lr.clone()), Some(rr.clone())),
                (Some(cr), None) | (None, Some(cr)) => rrc.right = Some(cr.clone()),
                (None, None) => {}
            }
            return Some(rr.clone());
        }
        None
    }
    if let Some(rr) = root {
        dfs(Some(rr.clone()));
    }
}
