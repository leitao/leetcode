// Definition for singly-linked list.

struct Solution;

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

    fn add(&self, n : &mut Box<ListNode>) {
        n.next = self.next;
        self.next = Some(n);
        
    }
}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let b = Box::new(ListNode::new(10));
        return Some(b)
    }
}

fn main() {
    let mut a = Some(Box::new(ListNode::new(1)));
    let mut b = Some(Box::new(ListNode::new(2)));

    Solution::add_two_numbers(a,b);
}
