// Tests for Problem 3614: Process String with Special Operations II
// Java reference: src/test/java/g3601_3700/s3614_process_string_with_special_operations_ii/SolutionTest.java
use leetcode_in_rust::s3614::process_string_with_special_operations_ii::Solution;
#[test]
fn test_process_str() { assert_eq!(Solution::process_str("a#b%*".to_string(), 1), 'a'); }
#[test]
fn test_process_str2() { assert_eq!(Solution::process_str("cd%#*#".to_string(), 3), 'd'); }
#[test]
fn test_process_str3() { assert_eq!(Solution::process_str("z*#".to_string(), 0), '.'); }
