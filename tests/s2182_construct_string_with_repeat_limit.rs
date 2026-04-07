// Tests for Problem 2182: Construct String With Repeat Limit
// Java reference: src/test/java/g2101_2200/s2182_construct_string_with_repeat_limit/SolutionTest.java

use leetcode_in_rust::s2182::construct_string_with_repeat_limit::Solution;

#[test]
fn test_repeat_limited_string() {
    assert_eq!(Solution::repeat_limited_string("cczazcc".to_string(), 3), "zzcccac");
}

#[test]
fn test_repeat_limited_string2() {
    assert_eq!(Solution::repeat_limited_string("aababab".to_string(), 2), "bbabaa");
}
