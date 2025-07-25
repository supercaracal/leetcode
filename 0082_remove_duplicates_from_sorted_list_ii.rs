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
    if args.len() != 2 {
        return Err("usage: 1,2,3,3,4,4,5");
    }
    let head = build_linked_list(parse_arg(&args[1]));
    println!("{:?}", delete_duplicates(head).unwrap());
    Ok(())
}

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(i32::MAX));
    dummy.next = head;
    let (root, _) = delete(Some(dummy));
    root.unwrap().next
}

fn delete(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, bool) {
    let mut is_dup = false;
    if let Some(mut node) = head {
        if let (Some(next), next_dup) = delete(node.next.take()) {
            node.next = if node.val == next.val {
                is_dup = true;
                next.next
            } else if next_dup {
                next.next
            } else {
                Some(next)
            };
        }
        return (Some(node), is_dup);
    }
    (None, is_dup)
}
