// Tests for Problem 0567: Permutation in String
// Java reference: src/test/java/g0501_0600/s0567_permutation_in_string/SolutionTest.java

use leetcode_in_rust::s0567::permutation_in_string::Solution;

#[test]
fn test_check_inclusion() {
    assert_eq!(Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()), true);
}

#[test]
fn test_check_inclusion2() {
    assert_eq!(Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()), false);
}
