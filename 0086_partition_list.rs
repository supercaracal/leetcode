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
        return Err("usage: 1,4,3,2,5,2 3");
    }
    let head = build_linked_list(parse_arg(&args[1]));
    let x = args[2].parse::<i32>().unwrap();
    println!("{:?}", partition(head, x).unwrap());
    Ok(())
}

fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    // TODO: fix
    if let Some(mut node) = head {
        if let Some(mut next) = partition(node.next.take(), x) {
            if node.val >= x && next.val < x {
                let n2 = next.next;
                node.next = n2;
                next.next = Some(node);
                return Some(next);
            } else {
                node.next = partition(Some(next), x);
                return Some(node);
            }
        }
        return Some(node);
    }
    None
}
