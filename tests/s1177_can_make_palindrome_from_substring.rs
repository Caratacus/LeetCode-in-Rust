// Tests for Problem 1177: Can Make Palindrome from Substring
// Java reference: src/test/java/g1101_1200/s1177_can_make_palindrome_from_substring/SolutionTest.java

use leetcode_in_rust::s1177::can_make_palindrome_from_substring::Solution;

#[test]
fn test_can_make_pali_queries() {
    assert_eq!(
        Solution::can_make_pali_queries(
            "abcda".to_string(),
            vec![vec![3, 3, 0], vec![1, 2, 0], vec![0, 3, 1], vec![0, 3, 2], vec![0, 4, 1]]
        ),
        vec![true, false, false, true, true]
    );
}

#[test]
fn test_can_make_pali_queries2() {
    assert_eq!(
        Solution::can_make_pali_queries("lyb  ".to_string(), vec![vec![0, 1, 0], vec![2, 2, 1]]),
        vec![false, true]
    );
}
