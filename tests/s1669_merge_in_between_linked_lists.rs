// Tests for Problem 1669: Merge In Between Linked Lists
// Java reference: src/test/java/g1601_1700/s1669_merge_in_between_linked_lists/SolutionTest.java

use leetcode_in_rust::s1669::merge_in_between_linked_lists::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_merge_in_between() {
    let list1 = linked_list_from_vec(vec![0, 1, 2, 3, 4, 5]);
    let list2 = linked_list_from_vec(vec![1000000, 1000001, 1000002]);
    let result = Solution::merge_in_between(list1, 3, 4, list2);
    assert_eq!(linked_list_to_vec(result), vec![0, 1, 2, 1000000, 1000001, 1000002, 5]);
}

#[test]
fn test_merge_in_between2() {
    let list1 = linked_list_from_vec(vec![0, 1, 2, 3, 4, 5, 6]);
    let list2 = linked_list_from_vec(vec![1000000, 1000001, 1000002, 1000003, 1000004]);
    let result = Solution::merge_in_between(list1, 2, 5, list2);
    assert_eq!(
        linked_list_to_vec(result),
        vec![0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6]
    );
}
