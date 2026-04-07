// Tests for Problem 2551: Put Marbles in Bags
// Java reference: src/test/java/g2501_2600/s2551_put_marbles_in_bags/SolutionTest.java
use leetcode_in_rust::s2551::put_marbles_in_bags::Solution;

#[test]
fn test_put_marbles() {
    assert_eq!(Solution::put_marbles(vec![1, 3, 5, 1], 2), 4);
}
#[test]
fn test_put_marbles2() {
    assert_eq!(Solution::put_marbles(vec![1, 3], 2), 0);
}
