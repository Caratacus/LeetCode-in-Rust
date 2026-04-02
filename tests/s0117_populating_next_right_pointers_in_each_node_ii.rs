// Tests for Problem 0117: Populating Next Right Pointers in Each Node II
// Java reference: src/test/java/g0101_0200/s0117_populating_next_right_pointers_in_each_node_ii/SolutionTest.java

use leetcode_in_rust::s0117::populating_next_right_pointers_in_each_node_ii::Solution;
use leetcode_in_rust::common::left_right_node::LeftRightNode;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_connect() {
    let root = None;
    let result = Solution::connect(root);
    assert!(result.is_none());
}

#[test]
fn test_connect2() {
    // Single node
    let root = Some(Rc::new(RefCell::new(LeftRightNode::new(1))));
    let result = Solution::connect(root);
    assert!(result.is_some());
}
