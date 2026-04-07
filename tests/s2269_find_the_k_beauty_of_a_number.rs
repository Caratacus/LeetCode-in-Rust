// Tests for Problem 2269: Find the K-Beauty of a Number
// Java reference: src/test/java/g2201_2300/s2269_find_the_k_beauty_of_a_number/SolutionTest.java

use leetcode_in_rust::s2269::find_the_k_beauty_of_a_number::Solution;

#[test]
fn test_divisor_substrings() {
    assert_eq!(Solution::divisor_substrings(240, 2), 2);
}

#[test]
fn test_divisor_substrings2() {
    assert_eq!(Solution::divisor_substrings(430043, 2), 2);
}
