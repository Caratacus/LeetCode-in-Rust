// Tests for Problem 3612: Process String with Special Operations I
// Java reference: src/test/java/g3601_3700/s3612_process_string_with_special_operations_i/SolutionTest.java
use leetcode_in_rust::s3612::process_string_with_special_operations_i::Solution;
#[test]
fn test_process_str() { assert_eq!(Solution::process_str("a#b%*".to_string()), "ba".to_string()); }
#[test]
fn test_process_str2() { assert_eq!(Solution::process_str("z*#".to_string()), "".to_string()); }
