// Tests for Problem 0148: Sort List
// Java reference: src/test/java/g0121_0200/s0148_sort_list/SolutionTest.java

use leetcode_in_rust::s0148::sort_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_sort_list() {
    let head = linked_list_from_vec(vec![4, 2, 1, 3]);
    let _result = Solution::sort_list(head);
    // Result should be 1->2->3->4
}
