// Tests for Problem 2213: Longest Substring of One Repeating Character
// Java reference: src/test/java/g2201_2300/s2213_longest_substring_of_one_repeating_character/SolutionTest.java

use leetcode_in_rust::s2213::longest_substring_of_one_repeating_character::Solution;

#[test]
fn test_longest_repeating() {
    assert_eq!(
        Solution::longest_repeating("babacc".to_string(), "bcb".to_string(), vec![1, 3, 3]),
        vec![3, 3, 4]
    );
}

#[test]
fn test_longest_repeating2() {
    assert_eq!(
        Solution::longest_repeating("abyzz".to_string(), "aa".to_string(), vec![2, 1]),
        vec![2, 3]
    );
}
