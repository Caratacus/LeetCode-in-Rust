// Tests for Problem 0458: Poor Pigs
// Java reference: src/test/java/g0401_0500/s0458_poor_pigs/SolutionTest.java

use leetcode_in_rust::s0458::poor_pigs::Solution;

#[test]
fn test_poor_pigs() {
    assert_eq!(Solution::poor_pigs(1000, 15, 60), 5);
}

#[test]
fn test_poor_pigs2() {
    assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
}

#[test]
fn test_poor_pigs3() {
    assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
}
