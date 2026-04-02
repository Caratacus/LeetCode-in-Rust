// Tests for Problem 0984: String Without AAA or BBB
// Java reference: src/test/java/g0901_1000/s0984_string_without_aaa_or_bbb/SolutionTest.java

use leetcode_in_rust::s0984::string_without_aaa_or_bbb::Solution;

#[test]
fn test_str_without3a3b() {
    assert_eq!(Solution::str_without3a3b(1, 2), "bba");
}

#[test]
fn test_str_without3a3b2() {
    assert_eq!(Solution::str_without3a3b(4, 1), "aabaa");
}
