// Tests for Problem 2816: Double a Number Represented as a Linked List
// Java reference: src/test/java/g2801_2900/s2816_double_a_number_represented_as_a_linked_list/SolutionTest.java

use leetcode_in_rust::s2816::double_a_number_represented_as_a_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_double_it() {
    let head = linked_list_from_vec(vec![1, 8, 9]);
    let result = Solution::double_it(head);
    assert_eq!(linked_list_to_vec(result), vec![3, 7, 8]);
}

#[test]
fn test_double_it2() {
    let head = linked_list_from_vec(vec![9, 9, 9]);
    let result = Solution::double_it(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 9, 9, 8]);
}
