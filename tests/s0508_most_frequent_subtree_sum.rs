// Tests for Problem 0508: Most Frequent Subtree Sum
// Java reference: src/test/java/g0501_0600/s0508_most_frequent_subtree_sum/SolutionTest.java

use leetcode_in_rust::s0508::most_frequent_subtree_sum::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_find_frequent_tree_sum() {
    let root = tree_from_vec(vec![Some(5), Some(2), Some(-3)]);
    let mut result = Solution::find_frequent_tree_sum(root);
    result.sort();
    assert_eq!(result, vec![-3, 2, 4]);
}

#[test]
fn test_find_frequent_tree_sum2() {
    let root = tree_from_vec(vec![Some(5), Some(2), Some(-5)]);
    let mut result = Solution::find_frequent_tree_sum(root);
    result.sort();
    assert_eq!(result, vec![2]);
}
