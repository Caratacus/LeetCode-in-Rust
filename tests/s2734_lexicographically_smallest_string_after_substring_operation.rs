// Tests for Problem 2734: Lexicographically Smallest String After Substring Operation
// Java reference: src/test/java/g2701_2800/s2734_lexicographically_smallest_string_after_substring_operation/SolutionTest.java

use leetcode_in_rust::s2734::lexicographically_smallest_string_after_substring_operation::Solution;

#[test]
fn test_smallest_string() {
    assert_eq!(Solution::smallest_string("cbabc".to_string()), "baabc");
}

#[test]
fn test_smallest_string2() {
    assert_eq!(Solution::smallest_string("acbbc".to_string()), "abaab");
}

#[test]
fn test_smallest_string3() {
    assert_eq!(Solution::smallest_string("leetcode".to_string()), "kddsbncd");
}
