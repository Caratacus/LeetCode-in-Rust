// Tests for Problem 2780: Minimum Index of a Valid Split
// Java reference: src/test/java/g2701_2800/s2780_minimum_index_of_a_valid_split/SolutionTest.java

use leetcode_in_rust::s2780::minimum_index_of_a_valid_split::Solution;

#[test]
fn test_minimum_index() {
    assert_eq!(Solution::minimum_index(vec![1, 2, 2, 2]), 2);
}

#[test]
fn test_minimum_index2() {
    assert_eq!(Solution::minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]), 4);
}

#[test]
fn test_minimum_index3() {
    assert_eq!(Solution::minimum_index(vec![3, 3, 3, 3, 7, 2, 2]), -1);
}
