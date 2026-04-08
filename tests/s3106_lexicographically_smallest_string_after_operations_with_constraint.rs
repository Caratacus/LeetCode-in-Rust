// Tests for Problem 3106: Lexicographically Smallest String After Operations With Constraint
// Java reference: src/test/java/g3101_3200/s3106_lexicographically_smallest_string_after_operations_with_constraint/SolutionTest.java

use leetcode_in_rust::s3106::lexicographically_smallest_string_after_operations_with_constraint::Solution;

#[test]
fn test_get_smallest_string() {
    assert_eq!(Solution::get_smallest_string("zbbz".to_string(), 3), "aaaz");
}

#[test]
fn test_get_smallest_string2() {
    assert_eq!(Solution::get_smallest_string("xaxcd".to_string(), 4), "aawcd");
}

#[test]
fn test_get_smallest_string3() {
    assert_eq!(Solution::get_smallest_string("lol".to_string(), 0), "lol");
}
