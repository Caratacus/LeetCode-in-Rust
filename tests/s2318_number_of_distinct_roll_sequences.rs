// Tests for Problem 2318: Number of Distinct Roll Sequences
// Java reference: src/test/java/g2301_2400/s2318_number_of_distinct_roll_sequences/SolutionTest.java

use leetcode_in_rust::s2318::number_of_distinct_roll_sequences::Solution;

#[test]
fn test_distinct_sequences() {
    assert_eq!(Solution::distinct_sequences(4), 184);
}

#[test]
fn test_distinct_sequences2() {
    assert_eq!(Solution::distinct_sequences(2), 22);
}
