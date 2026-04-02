// Tests for Problem 0445: Add Two Numbers II
// Java reference: src/test/java/g0401_0500/s0445_add_two_numbers_ii/SolutionTest.java

use leetcode_in_rust::s0445::add_two_numbers_ii::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_add_two_numbers() {
    let l1 = linked_list_from_vec(vec![7, 2, 4, 3]);
    let l2 = linked_list_from_vec(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![7, 8, 0, 7]);
}

#[test]
fn test_add_two_numbers2() {
    let l1 = linked_list_from_vec(vec![2, 4, 3]);
    let l2 = linked_list_from_vec(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![8, 0, 7]);
}

#[test]
fn test_add_two_numbers3() {
    let l1 = linked_list_from_vec(vec![0]);
    let l2 = linked_list_from_vec(vec![0]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![0]);
}
