// Tests for Problem 1290: Convert Binary Number in a Linked List to Integer
// Java reference: src/test/java/g1201_1300/s1290_convert_binary_number_in_a_linked_list_to_integer/SolutionTest.java

use leetcode_in_rust::s1290::convert_binary_number_in_a_linked_list_to_integer::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_get_decimal_value() {
    let head = linked_list_from_vec(vec![1, 0, 1]);
    assert_eq!(Solution::get_decimal_value(head), 5);
}

#[test]
fn test_get_decimal_value2() {
    let head = linked_list_from_vec(vec![0]);
    assert_eq!(Solution::get_decimal_value(head), 0);
}
