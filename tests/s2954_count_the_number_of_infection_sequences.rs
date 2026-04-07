// Tests for Problem 2954: Count the Number of Infection Sequences
// Java reference: src/test/java/g2901_3000/s2954_count_the_number_of_infection_sequences/SolutionTest.java

use leetcode_in_rust::s2954::count_the_number_of_infection_sequences::Solution;

#[test]
fn test_number_of_sequence() {
    assert_eq!(Solution::number_of_sequence(5, vec![0, 4]), 4);
}

#[test]
fn test_number_of_sequence2() {
    assert_eq!(Solution::number_of_sequence(4, vec![1]), 3);
}
