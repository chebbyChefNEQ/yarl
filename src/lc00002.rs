// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_impl(l1, l2, false)
    }

    fn add_two_numbers_impl(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: bool,
    ) -> Option<Box<ListNode>> {
        match (l1, l2, carry) {
            (Some(node), None, false) | (None, Some(node), false) => Some(node),
            (Some(node), None, true) | (None, Some(node), true) => {
                Self::add_two_numbers_impl(Some(node), Some(Box::new(ListNode::new(1))), false)
            }
            (None, None, false) => None,
            (None, None, true) => Some(Box::new(ListNode::new(1))),
            (Some(node1), Some(node2), carry) => {
                let sum = node1.val + node2.val + if carry { 1 } else { 0 };
                let mut node = ListNode::new(sum % 10);
                node.next = Self::add_two_numbers_impl(node1.next, node2.next, sum >= 10);
                Some(Box::new(node))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    fn make_chain(nums: Vec<i32>) -> Option<Box<super::ListNode>> {
        let mut head = None;
        for &n in nums.iter().rev() {
            let mut node = super::ListNode::new(n);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
    #[test]
    fn simple() {
        let res = super::Solution::add_two_numbers(
            make_chain(vec![9, 9, 9, 9, 9, 9, 9]),
            make_chain(vec![9, 9, 9, 9]),
        );
        println!("{:?}", res);
    }
}
