// Tests for Problem 0143: Reorder List
// Java reference: src/test/java/g0121_0200/s0143_reorder_list/SolutionTest.java

use leetcode_in_rust::s0143::reorder_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_reorder_list() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4]);
    Solution::reorder_list(head);
}

#[test]
fn test_reorder_list2() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    Solution::reorder_list(head);
}
