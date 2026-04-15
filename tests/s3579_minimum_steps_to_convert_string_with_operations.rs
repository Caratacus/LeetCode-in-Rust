// Tests for Problem 3579: Minimum Steps to Convert String with Operations
// Java reference: src/test/java/g3501_3600/s3579_minimum_steps_to_convert_string_with_operations/SolutionTest.java
use leetcode_in_rust::s3579::minimum_steps_to_convert_string_with_operations::Solution;
#[test] fn test_min_operations() { assert_eq!(Solution::min_operations("abcdf".to_string(), "dacbe".to_string()), 4); }
#[test] fn test_min_operations2() { assert_eq!(Solution::min_operations("abceded".to_string(), "baecfef".to_string()), 4); }
#[test] fn test_min_operations3() { assert_eq!(Solution::min_operations("abcdef".to_string(), "fedabc".to_string()), 2); }
