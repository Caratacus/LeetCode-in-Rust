// Tests for Problem 2825: Make String a Subsequence Using Cyclic Increments
// Java reference: src/test/java/g2801_2900/s2825_make_string_a_subsequence_using_cyclic_increments/SolutionTest.java

use leetcode_in_rust::s2825::make_string_a_subsequence_using_cyclic_increments::Solution;

#[test]
fn test_can_make_subsequence() {
    assert_eq!(
        Solution::can_make_subsequence("abc".to_string(), "ad".to_string()),
        true
    );
}

#[test]
fn test_can_make_subsequence2() {
    assert_eq!(
        Solution::can_make_subsequence("zc".to_string(), "ad".to_string()),
        true
    );
}

#[test]
fn test_can_make_subsequence3() {
    assert_eq!(
        Solution::can_make_subsequence("ab".to_string(), "d".to_string()),
        false
    );
}
