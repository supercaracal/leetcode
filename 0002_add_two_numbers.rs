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

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: 2,4,3 5,6,4");
    }
    let l1 = build_linked_list(parse_args(&args[1]));
    let l2 = build_linked_list(parse_args(&args[2]));
    println!("{:?}", add_two_numbers(l1, l2).unwrap());
    Ok(())
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

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut root: Option<Box<ListNode>> = None;
    let mut prev: Option<&mut ListNode> = None;
    let mut n1 = l1.as_ref();
    let mut n2 = l2.as_ref();
    let mut carry_up = false;
    loop {
        if n1.is_none() && n2.is_none() && !carry_up {
            break;
        }
        let mut sum = 0;
        sum += n1.map_or(0, |e| e.val);
        sum += n2.map_or(0, |e| e.val);
        if carry_up {
            sum += 1;
            carry_up = false;
        }
        if sum > 9 {
            carry_up = true;
        }
        let node = Box::new(ListNode::new(sum % 10));
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
        n1 = n1.and_then(|e| e.next.as_ref());
        n2 = n2.and_then(|e| e.next.as_ref());
    }
    root
}
