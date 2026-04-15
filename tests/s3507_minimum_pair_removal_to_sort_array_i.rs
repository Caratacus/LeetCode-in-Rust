// Tests for Problem 3507: Minimum Pair Removal to Sort Array I
// Java reference: src/test/java/g3501_3600/s3507_minimum_pair_removal_to_sort_array_i/SolutionTest.java

use leetcode_in_rust::s3507::minimum_pair_removal_to_sort_array_i::Solution;

#[test]
fn test_minimum_pair_removal() {
    assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
}

#[test]
fn test_minimum_pair_removal2() {
    assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
}
