// Tests for Problem 0725: Split Linked List in Parts
// Java reference: src/test/java/g0701_0800/s0725_split_linked_list_in_parts/SolutionTest.java

use leetcode_in_rust::s0725::split_linked_list_in_parts::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_split_list_to_parts() {
    let list = linked_list_from_vec(vec![1, 2, 3]);
    let result = Solution::split_list_to_parts(list, 5);
    let result: Vec<Vec<i32>> = result.into_iter().map(linked_list_to_vec).collect();

    assert_eq!(result.len(), 5);
    assert_eq!(result[0], vec![1]);
    assert_eq!(result[1], vec![2]);
    assert_eq!(result[2], vec![3]);
    assert_eq!(result[3], vec![]);
    assert_eq!(result[4], vec![]);
}

#[test]
fn test_split_list_to_parts2() {
    let list = linked_list_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let result = Solution::split_list_to_parts(list, 3);
    let result: Vec<Vec<i32>> = result.into_iter().map(linked_list_to_vec).collect();

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], vec![1, 2, 3, 4]);
    assert_eq!(result[1], vec![5, 6, 7]);
    assert_eq!(result[2], vec![8, 9, 10]);
}
