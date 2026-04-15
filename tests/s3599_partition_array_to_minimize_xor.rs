// Tests for Problem 3599: Partition Array to Minimize XOR
// Java reference: src/test/java/g3501_3600/s3599_partition_array_to_minimize_xor/SolutionTest.java
use leetcode_in_rust::s3599::partition_array_to_minimize_xor::Solution;
#[test] fn test_min_xor() { assert_eq!(Solution::min_xor(vec![1, 2, 3], 2), 1); }
#[test] fn test_min_xor2() { assert_eq!(Solution::min_xor(vec![2, 3, 3, 2], 3), 2); }
#[test] fn test_min_xor3() { assert_eq!(Solution::min_xor(vec![1, 1, 2, 3, 1], 2), 0); }
