// Tests for Problem 1356: Sort Integers by The Number of 1 Bits
// Java reference: src/test/java/g1301_1400/s1356_sort_integers_by_the_number_of_1_bits/SolutionTest.java

use leetcode_in_rust::s1356::sort_integers_by_the_number_of_1_bits::Solution;

#[test]
fn test_sort_by_bits() {
    assert_eq!(Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]), vec![0, 1, 2, 4, 8, 3, 5, 6, 7]);
}

#[test]
fn test_sort_by_bits2() {
    assert_eq!(Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]), vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);
}
