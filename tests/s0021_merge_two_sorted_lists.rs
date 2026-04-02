// Tests for Problem 0021: Merge Two Sorted Lists
// Java reference: src/test/java/g0001_0100/s0021_merge_two_sorted_lists/SolutionTest.java

use leetcode_in_rust::s0021::merge_two_sorted_lists::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_merge_two_lists() {
    let l1 = linked_list_from_vec(vec![1, 2, 4]);
    let l2 = linked_list_from_vec(vec![1, 3, 4]);
    let result = Solution::merge_two_lists(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![1, 1, 2, 3, 4, 4]);
}

#[test]
fn test_merge_two_lists2() {
    let l1 = linked_list_from_vec(vec![]);
    let l2 = linked_list_from_vec(vec![]);
    let result = Solution::merge_two_lists(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![]);
}

#[test]
fn test_merge_two_lists3() {
    let l1 = linked_list_from_vec(vec![]);
    let l2 = linked_list_from_vec(vec![0]);
    let result = Solution::merge_two_lists(l1, l2);
    assert_eq!(linked_list_to_vec(result), vec![0]);
}
