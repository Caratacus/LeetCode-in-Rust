// Tests for Problem 2531: Make Number of Distinct Characters Equal
// Java reference: src/test/java/g2501_2600/s2531_make_number_of_distinct_characters_equal/SolutionTest.java

use leetcode_in_rust::s2531::make_number_of_distinct_characters_equal::Solution;

#[test]
fn test_is_it_possible() {
    assert_eq!(Solution::is_it_possible("ac".to_string(), "b".to_string()), false);
}
#[test]
fn test_is_it_possible2() {
    assert_eq!(Solution::is_it_possible("abcc".to_string(), "aab".to_string()), true);
}
