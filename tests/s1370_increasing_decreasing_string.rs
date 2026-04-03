// Tests for Problem 1370: Increasing Decreasing String
// Java reference: src/test/java/g1301_1400/s1370_increasing_decreasing_string/SolutionTest.java

use leetcode_in_rust::s1370::increasing_decreasing_string::Solution;

#[test]
fn test_sort_string() {
    assert_eq!(Solution::sort_string("aaaabbbbcccc".to_string()), "abccbaabccba");
}

#[test]
fn test_sort_string2() {
    assert_eq!(Solution::sort_string("rat".to_string()), "art");
}
