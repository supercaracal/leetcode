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

fn parse_args(arg: &str) -> Vec<i32> {
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
    let head = build_linked_list(parse_args(&args[1]));
    let n = args[2].parse::<i32>().unwrap();
    println!("{:?}", remove_nth_from_end(head, n).unwrap());
    Ok(())
}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let (_, node) = walk_through(&mut head, n);
    node
}

fn walk_through(node: &mut Option<Box<ListNode>>, pos: i32) -> (i32, Option<Box<ListNode>>) {
    match node {
        Some(ref mut n) => {
            let (i, nd) = walk_through(&mut n.next, pos);
            n.next = nd;
            let nth = i + 1;
            if nth == pos {
                (nth, n.next.to_owned())
            } else {
                (nth, node.to_owned())
            }
        }
        None => (0, None),
    }
}
