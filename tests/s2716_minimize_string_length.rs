// Tests for Problem 2716: Minimize String Length
// Java reference: src/test/java/g2701_2800/s2716_minimize_string_length/SolutionTest.java

use leetcode_in_rust::s2716::minimize_string_length::Solution;

#[test]
fn test_minimized_string_length() {
    assert_eq!(Solution::minimized_string_length("aaabc".to_string()), 3);
}

#[test]
fn test_minimized_string_length2() {
    assert_eq!(Solution::minimized_string_length("cbbd".to_string()), 3);
}

#[test]
fn test_minimized_string_length3() {
    assert_eq!(Solution::minimized_string_length("dddaaa".to_string()), 2);
}
