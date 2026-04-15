// Tests for Problem 3389: Minimum Operations to Make Character Frequencies Equal
// Java reference: src/test/java/g3301_3400/s3389_minimum_operations_to_make_character_frequencies_equal/SolutionTest.java

use leetcode_in_rust::s3389::minimum_operations_to_make_character_frequencies_equal::Solution;

#[test]
fn test_make_string_good() {
    assert_eq!(Solution::make_string_good("acab".to_string()), 1);
}

#[test]
fn test_make_string_good2() {
    assert_eq!(Solution::make_string_good("wddw".to_string()), 0);
}

#[test]
fn test_make_string_good3() {
    assert_eq!(Solution::make_string_good("aaabc".to_string()), 2);
}
