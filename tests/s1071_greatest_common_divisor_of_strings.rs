// Tests for Problem 1071: Greatest Common Divisor of Strings
// Java reference: src/test/java/g1001_1100/s1071_greatest_common_divisor_of_strings/SolutionTest.java

use leetcode_in_rust::s1071::greatest_common_divisor_of_strings::Solution;

#[test]
fn test_gcd_of_strings() {
    assert_eq!(Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()), "ABC");
}

#[test]
fn test_gcd_of_strings2() {
    assert_eq!(Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()), "AB");
}

#[test]
fn test_gcd_of_strings3() {
    assert_eq!(Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()), "");
}
