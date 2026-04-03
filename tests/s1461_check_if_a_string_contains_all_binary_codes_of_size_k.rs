// Tests for Problem 1461: Check If a String Contains All Binary Codes of Size K
// Java reference: src/test/java/g1401_1500/s1461_check_if_a_string_contains_all_binary_codes_of_size_k/SolutionTest.java

use leetcode_in_rust::s1461::check_if_a_string_contains_all_binary_codes_of_size_k::Solution;

#[test]
fn test_has_all_codes() {
    assert_eq!(Solution::has_all_codes("00110110".to_string(), 2), true);
}

#[test]
fn test_has_all_codes2() {
    assert_eq!(Solution::has_all_codes("0110".to_string(), 1), true);
}

#[test]
fn test_has_all_codes3() {
    assert_eq!(Solution::has_all_codes("0110".to_string(), 2), false);
}
