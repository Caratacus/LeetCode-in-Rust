// Tests for Problem 2236: Root Equals Sum of Children
// Java reference: src/test/java/g2201_2300/s2236_root_equals_sum_of_children/SolutionTest.java

use leetcode_in_rust::s2236::root_equals_sum_of_children::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_check_tree() {
    let tree = tree_from_vec(vec![Some(10), Some(4), Some(6)]);
    assert_eq!(Solution::check_tree(tree), true);
}

#[test]
fn test_check_tree2() {
    let tree = tree_from_vec(vec![Some(5), Some(3), Some(1)]);
    assert_eq!(Solution::check_tree(tree), false);
}
