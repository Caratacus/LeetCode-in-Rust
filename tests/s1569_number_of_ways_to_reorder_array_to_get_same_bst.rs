// Tests for Problem 1569: Number of Ways to Reorder Array to Get Same BST
// Java reference: src/test/java/g1501_1600/s1569_number_of_ways_to_reorder_array_to_get_same_bst/SolutionTest.java

use leetcode_in_rust::s1569::number_of_ways_to_reorder_array_to_get_same_bst::Solution;

#[test]
fn test_num_of_ways() {
    assert_eq!(Solution::num_of_ways(vec![2, 1, 3]), 1);
}

#[test]
fn test_num_of_ways2() {
    assert_eq!(Solution::num_of_ways(vec![3, 4, 5, 1, 2]), 5);
}

#[test]
fn test_num_of_ways3() {
    assert_eq!(Solution::num_of_ways(vec![1, 2, 3]), 0);
}
