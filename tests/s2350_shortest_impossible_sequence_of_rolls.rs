// Tests for Problem 2350: Shortest Impossible Sequence of Rolls
// Java reference: src/test/java/g2301_2400/s2350_shortest_impossible_sequence_of_rolls/SolutionTest.java

use leetcode_in_rust::s2350::shortest_impossible_sequence_of_rolls::Solution;

#[test]
fn test_shortest_sequence() {
    assert_eq!(Solution::shortest_sequence(vec![4, 2, 1, 2, 3, 3, 2, 4, 1], 4), 3);
}

#[test]
fn test_shortest_sequence2() {
    assert_eq!(Solution::shortest_sequence(vec![1, 1, 2, 2], 2), 2);
}

#[test]
fn test_shortest_sequence3() {
    assert_eq!(Solution::shortest_sequence(vec![1, 1, 3, 2, 2, 2, 3, 3], 4), 1);
}
