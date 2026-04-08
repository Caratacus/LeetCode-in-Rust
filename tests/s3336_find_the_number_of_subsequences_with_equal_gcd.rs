// Tests for Problem 3336: Find the Number of Subsequences With Equal GCD
// Java reference: src/test/java/g3301_3400/s3336_find_the_number_of_subsequences_with_equal_gcd/SolutionTest.java

use leetcode_in_rust::s3336::find_the_number_of_subsequences_with_equal_gcd::Solution;

#[test]
fn test_subsequence_pair_count() {
    assert_eq!(Solution::subsequence_pair_count(vec![1, 2, 3, 4]), 10);
}

#[test]
fn test_subsequence_pair_count2() {
    assert_eq!(Solution::subsequence_pair_count(vec![10, 20, 30]), 2);
}
