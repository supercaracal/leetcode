#[derive(Clone)]
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
    let mut lists = Vec::with_capacity(args.len() - 1);
    for i in args.iter().skip(1) {
        let list = build_linked_list(parse_arg(i));
        lists.push(list)
    }
    println!("{:?}", merge_k_lists(lists).unwrap());
    Ok(())
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    while lists.len() > 1 {
        let list1 = lists.pop();
        let list2 = lists.pop();
        match (list1, list2) {
            (Some(l1), Some(l2)) => lists.push(merge_two_lists(l1, l2)),
            (Some(l), None) | (None, Some(l)) => lists.push(l),
            _ => {}
        }
    }
    lists.pop().and_then(|e| e)
}

fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(mut n1), Some(mut n2)) => {
            if n1.val <= n2.val {
                n1.next = merge_two_lists(n1.next, Some(n2));
                Some(n1)
            } else {
                n2.next = merge_two_lists(Some(n1), n2.next);
                Some(n2)
            }
        }
        (Some(n), None) | (None, Some(n)) => Some(n),
        _ => None,
    }
}
