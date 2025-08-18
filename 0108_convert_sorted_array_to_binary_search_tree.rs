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

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: -10,-3,0,5,9");
    }
    let nums = args[1]
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    println!("{:?}", sorted_array_to_bst(nums));
    Ok(())
}

fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let root_index = nums.len() / 2;
        let root_value = nums[root_index];
        let root = Rc::new(RefCell::new(TreeNode::new(root_value)));
        let rr = root.clone();
        let mut rrc = rr.borrow_mut();
        if root_index > 0 {
            rrc.left = dfs(&nums[..root_index]);
        }
        if root_index + 1 < nums.len() {
            rrc.right = dfs(&nums[root_index + 1..]);
        }
        Some(root)
    }
    dfs(&nums)
}
