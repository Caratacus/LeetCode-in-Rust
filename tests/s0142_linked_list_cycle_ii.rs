// Tests for Problem 0142: Linked List Cycle II
// Java reference: src/test/java/g0121_0200/s0142_linked_list_cycle_ii/SolutionTest.java

use leetcode_in_rust::s0142::linked_list_cycle_ii::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_detect_cycle() {
    let head = linked_list_from_vec(vec![3, 2, 0, -4]);
    // No cycle in this simple list
    let result = Solution::detect_cycle(head);
    assert!(result.is_none());
}

#[test]
fn test_detect_cycle2() {
    let head = linked_list_from_vec(vec![1]);
    let result = Solution::detect_cycle(head);
    assert!(result.is_none());
}
