// Tests for Problem 2573: Find the String With LCP
// Java reference: src/test/java/g2501_2600/s2573_find_the_string_with_lcp/SolutionTest.java

use leetcode_in_rust::s2573::find_the_string_with_lcp::Solution;

#[test]
fn test_find_the_string() {
    assert_eq!(
        Solution::find_the_string(vec![vec![4, 0, 2, 0], vec![0, 3, 0, 1], vec![2, 0, 2, 0], vec![0, 1, 0, 1]]),
        "abab"
    );
}

#[test]
fn test_find_the_string2() {
    assert_eq!(
        Solution::find_the_string(vec![vec![4, 3, 2, 1], vec![3, 3, 2, 1], vec![2, 2, 2, 1], vec![1, 1, 1, 1]]),
        "aaaa"
    );
}

#[test]
fn test_find_the_string3() {
    assert_eq!(
        Solution::find_the_string(vec![vec![4, 3, 2, 1], vec![3, 3, 2, 1], vec![2, 2, 2, 1], vec![1, 1, 1, 3]]),
        ""
    );
}
