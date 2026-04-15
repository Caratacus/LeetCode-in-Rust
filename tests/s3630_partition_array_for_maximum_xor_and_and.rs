// Tests for Problem 3630: Partition Array for Maximum XOR and AND
// Java reference: src/test/java/g3601_3700/s3630_partition_array_for_maximum_xor_and_and/SolutionTest.java
use leetcode_in_rust::s3630::partition_array_for_maximum_xor_and_and::Solution;
#[test]
fn test_maximize_xor_and_xor() { assert_eq!(Solution::maximize_xor_and_xor(vec![2, 3]), 5i64); }
#[test]
fn test_maximize_xor_and_xor2() { assert_eq!(Solution::maximize_xor_and_xor(vec![1, 3, 2]), 6i64); }
#[test]
fn test_maximize_xor_and_xor3() { assert_eq!(Solution::maximize_xor_and_xor(vec![2, 3, 6, 7]), 15i64); }
