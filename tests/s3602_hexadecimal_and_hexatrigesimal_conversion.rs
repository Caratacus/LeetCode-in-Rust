// Tests for Problem 3602: Hexadecimal and Hexatrigesimal Conversion
// Java reference: src/test/java/g3601_3700/s3602_hexadecimal_and_hexatrigesimal_conversion/SolutionTest.java
use leetcode_in_rust::s3602::hexadecimal_and_hexatrigesimal_conversion::Solution;
#[test] fn test_concat_hex36() { assert_eq!(Solution::concat_hex36(13), "A91P1".to_string()); }
#[test] fn test_concat_hex362() { assert_eq!(Solution::concat_hex36(36), "5101000".to_string()); }
