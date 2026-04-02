// Tests for Problem 0438: Find All Anagrams in a String
// Java reference: src/test/java/g0401_0500/s0438_find_all_anagrams_in_a_string/SolutionTest.java

use leetcode_in_rust::s0438::find_all_anagrams_in_a_string::Solution;

#[test]
fn test_find_anagrams() {
    assert_eq!(Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()), vec![0, 6]);
}

#[test]
fn test_find_anagrams2() {
    assert_eq!(Solution::find_anagrams("abab".to_string(), "ab".to_string()), vec![0, 1, 2]);
}
