// Tests for Problem 3718: Smallest Missing Multiple of K
// Java reference: src/test/java/g3701_3800/s3718_smallest_missing_multiple_of_k/SolutionTest.java
use leetcode_in_rust::s3718::smallest_missing_multiple_of_k::Solution;
#[test]
fn test_missing_multiple() { assert_eq!(Solution::missing_multiple(vec![8, 2, 3, 4, 6], 2), 10); }
#[test]
fn test_missing_multiple2() { assert_eq!(Solution::missing_multiple(vec![1, 4, 7, 10, 15], 5), 5); }
