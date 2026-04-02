// Tests for Problem 0002: Add Two Numbers
// Java reference: src/test/java/g0001_0100/s0002_add_two_numbers/SolutionTest.java

use leetcode_in_rust::s0002::add_two_numbers::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_add_two_numbers() {
    let l1 = linked_list_from_vec(vec![2, 4, 3]);
    let l2 = linked_list_from_vec(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![7, 0, 8]);
}

#[test]
fn test_add_two_numbers2() {
    let l1 = linked_list_from_vec(vec![0]);
    let l2 = linked_list_from_vec(vec![0]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![0]);
}

#[test]
fn test_add_two_numbers3() {
    let l1 = linked_list_from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = linked_list_from_vec(vec![9, 9, 9, 9]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
}
