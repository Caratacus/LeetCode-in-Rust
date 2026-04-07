// Tests for Problem 2839: Check if Strings Can be Made Equal with Operations I
// Java reference: src/test/java/g2801_2900/s2839_check_if_strings_can_be_made_equal_with_operations_i/SolutionTest.java

use leetcode_in_rust::s2839::check_if_strings_can_be_made_equal_with_operations_i::Solution;

#[test]
fn test_can_be_equal() {
    assert_eq!(Solution::can_be_equal("abcd".to_string(), "cdab".to_string()), true);
}

#[test]
fn test_can_be_equal3() {
    assert_eq!(Solution::can_be_equal("abcd".to_string(), "abcd".to_string()), true);
}

#[test]
fn test_can_be_equal4() {
    assert_eq!(Solution::can_be_equal("abcd".to_string(), "cbad".to_string()), true);
}

#[test]
fn test_can_be_equal5() {
    assert_eq!(Solution::can_be_equal("abcd".to_string(), "adcb".to_string()), true);
}

#[test]
fn test_can_be_equal6() {
    assert_eq!(Solution::can_be_equal("abcd".to_string(), "abdc".to_string()), false);
}

#[test]
fn test_can_be_equal7() {
    assert_eq!(Solution::can_be_equal("abcd".to_string(), "wxyz".to_string()), false);
}

#[test]
fn test_can_be_equal8() {
    assert_eq!(Solution::can_be_equal("aabb".to_string(), "bbaa".to_string()), true);
}

#[test]
fn test_can_be_equal10() {
    assert_eq!(Solution::can_be_equal("abba".to_string(), "baab".to_string()), true);
}
