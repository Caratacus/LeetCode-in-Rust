// Tests for Problem 3686: Number of Stable Subsequences
// Java reference: src/test/java/g3601_3700/s3686_number_of_stable_subsequences/SolutionTest.java
use leetcode_in_rust::s3686::number_of_stable_subsequences::Solution;
#[test]
fn test_count_stable_subsequences() { assert_eq!(Solution::count_stable_subsequences(vec![1, 3, 5]), 6); }
#[test]
fn test_count_stable_subsequences2() { assert_eq!(Solution::count_stable_subsequences(vec![2, 3, 4, 2]), 14); }
