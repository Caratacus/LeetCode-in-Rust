// Tests for Problem 0761: Special Binary String
// Java reference: src/test/java/g0701_0800/s0761_special_binary_string/SolutionTest.java

use leetcode_in_rust::s0761::special_binary_string::Solution;

#[test]
fn test_make_largest_special() {
    assert_eq!(Solution::make_largest_special("11011000".to_string()), "11100100");
}

#[test]
fn test_make_largest_special2() {
    assert_eq!(Solution::make_largest_special("10".to_string()), "10");
}
