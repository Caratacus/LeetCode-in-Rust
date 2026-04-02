// Tests for Problem 0141: Linked List Cycle
// Java reference: src/test/java/g0121_0200/s0141_linked_list_cycle/SolutionTest.java

use leetcode_in_rust::s0141::linked_list_cycle::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_has_cycle() {
    let head = linked_list_from_vec(vec![3, 2, 0, -4]);
    assert_eq!(Solution::has_cycle(head), false);
}

#[test]
fn test_has_cycle2() {
    let head = linked_list_from_vec(vec![1, 2]);
    assert_eq!(Solution::has_cycle(head), false);
}

#[test]
fn test_has_cycle3() {
    let head = linked_list_from_vec(vec![1]);
    assert_eq!(Solution::has_cycle(head), false);
}
