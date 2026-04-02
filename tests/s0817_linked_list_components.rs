// Tests for Problem 0817: Linked List Components
// Java reference: src/test/java/g0801_0900/s0817_linked_list_components/SolutionTest.java

use leetcode_in_rust::s0817::linked_list_components::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_num_components() {
    let list = linked_list_from_vec(vec![0, 1, 2, 3]);
    assert_eq!(Solution::num_components(list, vec![0, 1, 3]), 2);
}

#[test]
fn test_num_components2() {
    let list = linked_list_from_vec(vec![0, 1, 2, 3, 4]);
    assert_eq!(Solution::num_components(list, vec![0, 3, 1, 4]), 2);
}
