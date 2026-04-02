// Tests for Problem 0067: Add Binary
// Java reference: src/test/java/g0001_0100/s0067_add_binary/SolutionTest.java

use leetcode_in_rust::s0067::add_binary::Solution;

#[test]
fn test_add_binary() {
    assert_eq!(Solution::add_binary("11".to_string(), "1".to_string()), "100");
}

#[test]
fn test_add_binary2() {
    assert_eq!(Solution::add_binary("1010".to_string(), "1011".to_string()), "10101");
}
