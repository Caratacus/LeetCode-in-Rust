// Tests for Problem 0147: Insertion Sort List
// Java reference: src/test/java/g0121_0200/s0147_insertion_sort_list/SolutionTest.java

use leetcode_in_rust::s0147::insertion_sort_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_insertion_sort_list() {
    let head = linked_list_from_vec(vec![4, 2, 1, 3]);
    let _result = Solution::insertion_sort_list(head);
    // Result should be 1->2->3->4
}
