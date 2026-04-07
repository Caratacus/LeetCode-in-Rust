// Tests for Problem 2423: Remove Letter To Equalize Frequency
// Java reference: src/test/java/g2401_2500/s2423_remove_letter_to_equalize_frequency/SolutionTest.java

use leetcode_in_rust::s2423::remove_letter_to_equalize_frequency::Solution;

#[test]
fn test_equal_frequency() {
    assert_eq!(Solution::equal_frequency(String::from("abcc")), true);
}

#[test]
fn test_equal_frequency2() {
    assert_eq!(Solution::equal_frequency(String::from("aazz")), false);
}
