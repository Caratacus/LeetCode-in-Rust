// Tests for Problem 3574: Maximize Subarray GCD Score
// Java reference: src/test/java/g3501_3600/s3574_maximize_subarray_gcd_score/SolutionTest.java
use leetcode_in_rust::s3574::maximize_subarray_gcd_score::Solution;
#[test] fn test_max_gcd_score() { assert_eq!(Solution::max_gcd_score(vec![2, 4], 1), 8i64); }
#[test] fn test_max_gcd_score2() { assert_eq!(Solution::max_gcd_score(vec![3, 5, 7], 2), 14i64); }
#[test] fn test_max_gcd_score3() { assert_eq!(Solution::max_gcd_score(vec![5, 5, 5], 1), 15i64); }
