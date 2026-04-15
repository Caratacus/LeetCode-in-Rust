// Tests for Problem 3510: Minimum Pair Removal to Sort Array II
// Java reference: src/test/java/g3501_3600/s3510_minimum_pair_removal_to_sort_array_ii/SolutionTest.java

use leetcode_in_rust::s3510::minimum_pair_removal_to_sort_array_ii::Solution;

#[test]
fn test_minimum_pair_removal() {
    assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
}

#[test]
fn test_minimum_pair_removal2() {
    assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
}

#[test]
fn test_minimum_pair_removal4() {
    assert_eq!(Solution::minimum_pair_removal(vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1]), 9);
}

#[test]
fn test_minimum_pair_removal5() {
    assert_eq!(Solution::minimum_pair_removal(vec![5]), 0);
}
