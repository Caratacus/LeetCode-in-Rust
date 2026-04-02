// Tests for Problem 0328: Odd Even Linked List
// Java reference: src/test/java/g0301_0400/s0328_odd_even_linked_list/SolutionTest.java

use leetcode_in_rust::s0328::odd_even_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_odd_even_list() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::odd_even_list(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 3, 5, 2, 4]);
}

#[test]
fn test_odd_even_list2() {
    let head = linked_list_from_vec(vec![2, 1, 3, 5, 6, 4, 7]);
    let result = Solution::odd_even_list(head);
    assert_eq!(linked_list_to_vec(result), vec![2, 3, 6, 7, 1, 5, 4]);
}
