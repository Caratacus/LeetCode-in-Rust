// Tests for Problem 1315: Sum of Nodes with Even-Valued Grandparent
// Java reference: src/test/java/g1301_1400/s1315_sum_of_nodes_with_even_valued_grandparent/SolutionTest.java

use leetcode_in_rust::s1315::sum_of_nodes_with_even_valued_grandparent::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_sum_even_grandparent() {
    let root = tree_from_vec(vec![Some(6), Some(7), Some(8), Some(2), Some(7), Some(1), Some(3), Some(9), None, Some(1), Some(4), None, None, None, Some(5)]);
    let result = Solution::sum_even_grandparent(root);
    assert_eq!(result, 18);
}

#[test]
fn test_sum_even_grandparent2() {
    let root = tree_from_vec(vec![Some(1)]);
    let result = Solution::sum_even_grandparent(root);
    assert_eq!(result, 0);
}
