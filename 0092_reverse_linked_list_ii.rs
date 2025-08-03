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
    if args.len() != 4 {
        return Err("usage: 1,2,3,4,5 2 4");
    }
    let head = build_linked_list(parse_arg(&args[1]));
    let left = args[2].parse::<i32>().unwrap();
    let right = args[3].parse::<i32>().unwrap();
    println!("{:?}", reverse_between(head, left, right).unwrap());
    Ok(())
}

fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    // TODO: fix
    // I've got lost ways to manipulate nodes in Rust.
    println!("{left}, {right}");
    let mut dummy = Box::new(ListNode::new(-1));
    dummy.next = head;
    dummy.next
}
