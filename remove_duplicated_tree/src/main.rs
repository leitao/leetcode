// use std::mem;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn append(&mut self, next: Box<ListNode>) {
        self.next = Some(next);
    }

    fn print(self) {
        print!("value={} ", self.val);
        if self.next.is_some() {
            let n = self.next.unwrap();
            n.print();
        } else {
            println!();
        }
    }
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list : Box<ListNode>;
        let mut tail;

        if head.is_none() {
            return None;
        }

        let mut c = head.unwrap();

        // First element always make the list
        new_list = Box::new(ListNode::new(c.val));

        tail = new_list.as_mut();

        loop {
            if c.next.is_none() {
                return Some(new_list);
            }
            let n = c.next.unwrap();
            if n.val == c.val {
                c = n;
                continue;
            }
            let new_node = Box::new(ListNode::new(n.val));
            tail.next = Some(new_node);
            // tail.append(new_node);

            tail = tail.next.as_mut().unwrap();
            c = n;
        }


    }
}

pub fn print(head: Option<Box<ListNode>>) {
    println!("Head is {:?}", head);
    if head.is_some() {
        head.unwrap().print();
    }
}

fn main() {
    let mut node1 = Box::new(ListNode::new(9));
    let mut node2 = Box::new(ListNode::new(12));
    let mut node3 = Box::new(ListNode::new(13));
    let node4 = Box::new(ListNode::new(11));

    node3.append(node4);
    node2.append(node3);
    node1.append(node2);

    let n = Solution::delete_duplicates(Some(node1));

    print(n);
}
