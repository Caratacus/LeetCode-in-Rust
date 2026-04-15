// Tests for Problem 3428: Maximum and Minimum Sums of at Most Size K Subsequences
// Java reference: src/test/java/g3401_3500/s3428_maximum_and_minimum_sums_of_at_most_size_k_subsequences/SolutionTest.java

use leetcode_in_rust::s3428::maximum_and_minimum_sums_of_at_most_size_k_subsequences::Solution;

#[test]
fn test_min_max_sums() {
    assert_eq!(Solution::min_max_sums(vec![1, 2, 3], 2), 24);
}

#[test]
fn test_min_max_sums2() {
    assert_eq!(Solution::min_max_sums(vec![5, 0, 6], 1), 22);
}
