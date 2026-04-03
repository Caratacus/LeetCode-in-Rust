// Tests for Problem 1805: Number of Different Integers in a String
// Java reference: src/test/java/g1801_1900/s1805_number_of_different_integers_in_a_string/SolutionTest.java

use leetcode_in_rust::s1805::number_of_different_integers_in_a_string::Solution;

#[test]
fn test_num_different_integers() {
    assert_eq!(Solution::num_different_integers("a123bc34d8ef34".to_string()), 3);
}

#[test]
fn test_num_different_integers2() {
    assert_eq!(Solution::num_different_integers("leet1234code234".to_string()), 2);
}

#[test]
fn test_num_different_integers3() {
    assert_eq!(Solution::num_different_integers("a1b01c001".to_string()), 1);
}
