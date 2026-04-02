// Tests for Problem 0234: Palindrome Linked List
// Java reference: src/test/java/g0201_0300/s0234_palindrome_linked_list/SolutionTest.java

use leetcode_in_rust::s0234::palindrome_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_is_palindrome() {
    let head = linked_list_from_vec(vec![1, 2, 2, 1]);
    assert_eq!(Solution::is_palindrome(head), true);
}

#[test]
fn test_is_palindrome2() {
    let head = linked_list_from_vec(vec![1, 2]);
    assert_eq!(Solution::is_palindrome(head), false);
}
