// Tests for Problem 1305: All Elements in Two Binary Search Trees
// Java reference: src/test/java/g1301_1400/s1305_all_elements_in_two_binary_search_trees/SolutionTest.java

use leetcode_in_rust::s1305::all_elements_in_two_binary_search_trees::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_get_all_elements() {
    let root1 = tree_from_vec(vec![Some(2), Some(1), Some(4)]);
    let root2 = tree_from_vec(vec![Some(1), Some(0), Some(3)]);
    assert_eq!(Solution::get_all_elements(root1, root2), vec![0, 1, 1, 2, 3, 4]);
}

#[test]
fn test_get_all_elements2() {
    let root1 = tree_from_vec(vec![Some(1), None, Some(8)]);
    let root2 = tree_from_vec(vec![Some(8), Some(1)]);
    assert_eq!(Solution::get_all_elements(root1, root2), vec![1, 1, 8, 8]);
}
