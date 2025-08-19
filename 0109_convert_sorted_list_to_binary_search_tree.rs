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

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        let mut node = self;
        loop {
            list.entry(&node.val);
            if node.next.is_none() {
                break;
            }
            node = node.next.as_ref().unwrap();
        }
        list.finish()
    }
}

fn parse_arg(arg: &str) -> Vec<i32> {
    arg.split(',').map(|e| e.parse::<i32>().unwrap()).collect()
}

fn build_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut root: Option<Box<ListNode>> = None;
    let mut prev: Option<&mut ListNode> = None;
    for n in nums {
        let node = Box::new(ListNode::new(n));
        match prev {
            Some(p) => {
                p.next = Some(node);
                prev = Some(p.next.as_mut().unwrap());
            }
            None => {
                root = Some(node);
                prev = Some(root.as_mut().unwrap());
            }
        }
    }
    root
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: -10,-3,0,5,9");
    }
    let head = build_linked_list(parse_arg(&args[1]));
    println!("{:?}", sorted_list_to_bst(head));
    Ok(())
}

fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let list_size = list_len(&head);
    let mut head = head;
    if let Some(ref mut n) = split_list(&mut head, 0, list_size / 2) {
        let root = Rc::new(RefCell::new(TreeNode::new(n.val)));
        let rr = root.clone();
        let mut rrc = rr.borrow_mut();
        rrc.left = sorted_list_to_bst(head);
        rrc.right = sorted_list_to_bst(n.next.take());
        return Some(root.clone());
    }
    None
}

fn list_len(head: &Option<Box<ListNode>>) -> usize {
    if let Some(node) = head {
        return 1 + list_len(&node.next);
    }
    0
}

fn split_list(head: &mut Option<Box<ListNode>>, i: usize, pos: usize) -> Option<Box<ListNode>> {
    if let Some(node) = head {
        if i == 0 && pos == 0 {
            return head.take();
        }
        if i == pos.checked_sub(1).map_or(0, |v| v) {
            return node.next.take();
        }
        return split_list(&mut node.next, i + 1, pos);
    }
    None
}
