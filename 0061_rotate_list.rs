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
    if args.len() != 3 {
        return Err("usage: 1,2,3,4,5 2");
    }
    let head = build_linked_list(parse_arg(&args[1]));
    let k = args[2].parse::<i32>().unwrap();
    println!("{:?}", rotate_right(head, k).unwrap());
    Ok(())
}

fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k == 0 {
        return head;
    }
    let size = size(&head);
    match size {
        0 => return None,
        1 => return head,
        n if k % n == 0 => return head,
        _ => {}
    }
    let tail_idx = size - (k % size);
    let mut head = head;
    let mut new_head = split(&mut head, tail_idx);
    concat(&mut new_head, head);
    new_head
}

fn size(head: &Option<Box<ListNode>>) -> i32 {
    if let Some(ref node) = head {
        1 + size(&node.next)
    } else {
        0
    }
}

fn split(head: &mut Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if let Some(h) = head {
        if k == 1 {
            h.next.take()
        } else {
            split(&mut h.next, k - 1)
        }
    } else {
        None
    }
}

fn concat(a: &mut Option<Box<ListNode>>, b: Option<Box<ListNode>>) {
    if let Some(node) = a {
        if node.next.is_none() {
            node.next = b;
            return;
        } else {
            concat(&mut node.next, b);
        }
    }
}
