// Tests for Problem 1647: Minimum Deletions to Make Character Frequencies Unique
// Java reference: src/test/java/g1601_1700/s1647_minimum_deletions_to_make_character_frequencies_unique/SolutionTest.java

use leetcode_in_rust::s1647::minimum_deletions_to_make_character_frequencies_unique::Solution;

#[test]
fn test_min_deletions() {
    assert_eq!(Solution::min_deletions("aab".to_string()), 0);
}

#[test]
fn test_min_deletions2() {
    assert_eq!(Solution::min_deletions("aaabbbcc".to_string()), 2);
}

#[test]
fn test_min_deletions3() {
    assert_eq!(Solution::min_deletions("ceabaacb".to_string()), 2);
}
