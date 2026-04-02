// Tests for Problem 0023: Merge k Sorted Lists
// Java reference: src/test/java/g0001_0100/s0023_merge_k_sorted_lists/SolutionTest.java

use leetcode_in_rust::s0023::merge_k_sorted_lists::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_merge_k_lists() {
    let lists = vec![
        linked_list_from_vec(vec![1, 4, 5]),
        linked_list_from_vec(vec![1, 3, 4]),
        linked_list_from_vec(vec![2, 6]),
    ];
    let result = Solution::merge_k_lists(lists);
    assert_eq!(linked_list_to_vec(result), vec![1, 1, 2, 3, 4, 4, 5, 6]);
}

#[test]
fn test_merge_k_lists2() {
    let lists: Vec<_> = vec![];
    let result = Solution::merge_k_lists(lists);
    assert_eq!(linked_list_to_vec(result), vec![]);
}

#[test]
fn test_merge_k_lists3() {
    let lists = vec![linked_list_from_vec(vec![])];
    let result = Solution::merge_k_lists(lists);
    assert_eq!(linked_list_to_vec(result), vec![]);
}
