// Tests for Problem 1704: Determine if String Halves Are Alike
// Java reference: src/test/java/g1701_1800/s1704_determine_if_string_halves_are_alike/SolutionTest.java

use leetcode_in_rust::s1704::determine_if_string_halves_are_alike::Solution;

#[test]
fn test_halves_are_alike() {
    assert_eq!(Solution::halves_are_alike("book".to_string()), true);
}

#[test]
fn test_halves_are_alike2() {
    assert_eq!(Solution::halves_are_alike("textbook".to_string()), false);
}

#[test]
fn test_halves_are_alike4() {
    assert_eq!(Solution::halves_are_alike("aeiouAEIOU".to_string()), true);
}
