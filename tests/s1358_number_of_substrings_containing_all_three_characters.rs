// Tests for Problem 1358: Number of Substrings Containing All Three Characters
// Java reference: src/test/java/g1301_1400/s1358_number_of_substrings_containing_all_three_characters/SolutionTest.java

use leetcode_in_rust::s1358::number_of_substrings_containing_all_three_characters::Solution;

#[test]
fn test_number_of_substrings() {
    assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
}

#[test]
fn test_number_of_substrings2() {
    assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3);
}

#[test]
fn test_number_of_substrings3() {
    assert_eq!(Solution::number_of_substrings("abc".to_string()), 1);
}
