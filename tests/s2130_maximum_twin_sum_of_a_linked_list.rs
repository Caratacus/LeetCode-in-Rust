// Tests for Problem 2130: Maximum Twin Sum of a Linked List
// Java reference: src/test/java/g2101_2200/s2130_maximum_twin_sum_of_a_linked_list/SolutionTest.java

use leetcode_in_rust::s2130::maximum_twin_sum_of_a_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_pair_sum() {
    let head = linked_list_from_vec(vec![5, 4, 2, 1]);
    assert_eq!(Solution::pair_sum(head), 6);
}

#[test]
fn test_pair_sum2() {
    let head = linked_list_from_vec(vec![4, 2, 2, 3]);
    assert_eq!(Solution::pair_sum(head), 7);
}

#[test]
fn test_pair_sum3() {
    let head = linked_list_from_vec(vec![1, 100000]);
    assert_eq!(Solution::pair_sum(head), 100001);
}
