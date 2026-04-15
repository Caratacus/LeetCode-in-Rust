// Tests for Problem 3597: Partition String
// Java reference: src/test/java/g3501_3600/s3597_partition_string/SolutionTest.java
use leetcode_in_rust::s3597::partition_string::Solution;
#[test] fn test_partition_string() { assert_eq!(Solution::partition_string("abbccccd".to_string()), vec!["a".to_string(), "b".to_string(), "bc".to_string(), "c".to_string(), "cc".to_string(), "d".to_string()]); }
#[test] fn test_partition_string2() { assert_eq!(Solution::partition_string("aaaa".to_string()), vec!["a".to_string(), "aa".to_string()]); }
