// Tests for Problem 1208: Get Equal Substrings Within Budget
// Java reference: src/test/java/g1201_1300/s1208_get_equal_substrings_within_budget/SolutionTest.java

use leetcode_in_rust::s1208::get_equal_substrings_within_budget::Solution;

#[test]
fn test_equal_substring() {
    let result = Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3);
    assert_eq!(result, 3);
}

#[test]
fn test_equal_substring2() {
    let result = Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3);
    assert_eq!(result, 1);
}

#[test]
fn test_equal_substring3() {
    let result = Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0);
    assert_eq!(result, 1);
}
