// Tests for Problem 3217: Delete Nodes From Linked List Present in Array
// Java reference: src/test/java/g3201_3300/s3217_delete_nodes_from_linked_list_present_in_array/SolutionTest.java

use leetcode_in_rust::s3217::delete_nodes_from_linked_list_present_in_array::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_modified_list() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::modified_list(vec![1, 2, 3], head);
    assert_eq!(linked_list_to_vec(result), vec![4, 5]);
}

#[test]
fn test_modified_list2() {
    let head = linked_list_from_vec(vec![1, 2, 1, 2, 1, 2]);
    let result = Solution::modified_list(vec![1], head);
    assert_eq!(linked_list_to_vec(result), vec![2, 2, 2]);
}

#[test]
fn test_modified_list3() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4]);
    let result = Solution::modified_list(vec![5], head);
    assert_eq!(linked_list_to_vec(result), vec![1, 2, 3, 4]);
}
