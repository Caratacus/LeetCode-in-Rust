// Tests for Problem 3628: Maximum Number of Subsequences After One Inserting
// Java reference: src/test/java/g3601_3700/s3628_maximum_number_of_subsequences_after_one_inserting/SolutionTest.java
use leetcode_in_rust::s3628::maximum_number_of_subsequences_after_one_inserting::Solution;
#[test]
fn test_num_of_subsequences() { assert_eq!(Solution::num_of_subsequences("LMCT".to_string()), 2i64); }
#[test]
fn test_num_of_subsequences2() { assert_eq!(Solution::num_of_subsequences("LCCT".to_string()), 4i64); }
#[test]
fn test_num_of_subsequences3() { assert_eq!(Solution::num_of_subsequences("L".to_string()), 0i64); }
