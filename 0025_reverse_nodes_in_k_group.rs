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
    println!("{:?}", reverse_k_group(head, k).unwrap());
    Ok(())
}

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let k = k as usize;
    let mut stacks = Vec::new();
    while head.is_some() {
        let mut i = 0;
        let mut stack = Vec::with_capacity(k);
        while i < k {
            if let Some(mut h) = head {
                let next = h.next.take();
                stack.push(Some(h));
                head = next;
            }
            i += 1;
        }
        stacks.push(stack);
    }
    let mut root: Option<Box<ListNode>> = None;
    let mut prev: Option<&mut Box<ListNode>> = None;
    for mut stack in stacks {
        if stack.len() == k {
            while let Some(e) = stack.pop() {
                if let Some(node) = e {
                    if let Some(p) = prev {
                        p.next = Some(node);
                        prev = p.next.as_mut();
                    } else {
                        root = Some(node);
                        prev = root.as_mut();
                    }
                }
            }
        } else {
            for node in stack {
                if let Some(node) = node {
                    if let Some(p) = prev {
                        p.next = Some(node);
                        prev = p.next.as_mut();
                    } else {
                        root = Some(node);
                        prev = root.as_mut();
                    }
                }
            }
        }
    }
    root
}
