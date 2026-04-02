// Problem 0002: add two numbers
// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Math #Linked_List #Recursion
// #Data_Structure_II_Day_10_Linked_List #Programming_Skills_II_Day_15
// #Top_Interview_150_Linked_List #Big_O_Time_O(max(N,M))_Space_O(max(N,M))

use crate::common::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0, None));
        let mut current = &mut dummy_head;
        let mut p = l1;
        let mut q = l2;
        let mut carry = 0;

        while p.is_some() || q.is_some() {
            let x = if let Some(node) = p {
                let val = node.val;
                p = node.next;
                val
            } else {
                0
            };
            let y = if let Some(node) = q {
                let val = node.val;
                q = node.next;
                val
            } else {
                0
            };
            let sum = carry + x + y;
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10, None)));
            current = current.next.as_mut().unwrap();
        }

        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry, None)));
        }

        dummy_head.next
    }
}

