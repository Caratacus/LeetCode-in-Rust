// Tests for Problem 0086: Partition List
// Java reference: src/test/java/g0001_0100/s0086_partition_list/SolutionTest.java

use leetcode_in_rust::s0086::partition_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_partition() {
    let head = linked_list_from_vec(vec![1, 4, 3, 2, 5, 2]);
    let result = Solution::partition(head, 3);
    assert_eq!(linked_list_to_vec(result), vec![1, 2, 2, 4, 3, 5]);
}

#[test]
fn test_partition2() {
    let head = linked_list_from_vec(vec![2, 1]);
    let result = Solution::partition(head, 2);
    assert_eq!(linked_list_to_vec(result), vec![1, 2]);
}
