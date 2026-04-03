// Tests for Problem 1339: Maximum Product of Splitted Binary Tree
// Java reference: src/test/java/g1301_1400/s1339_maximum_product_of_splitted_binary_tree/SolutionTest.java

use leetcode_in_rust::s1339::maximum_product_of_splitted_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_max_product() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    let result = Solution::max_product(root);
    assert_eq!(result, 110);
}

#[test]
fn test_max_product2() {
    let root = tree_from_vec(vec![Some(1), None, Some(2), Some(3), Some(4), None, None, Some(5), Some(6)]);
    let result = Solution::max_product(root);
    assert_eq!(result, 90);
}
