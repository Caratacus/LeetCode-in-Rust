// Tests for Problem 0741: Cherry Pickup
// Java reference: src/test/java/g0701_0800/s0741_cherry_pickup/SolutionTest.java

use leetcode_in_rust::s0741::cherry_pickup::Solution;

#[test]
fn test_cherry_pickup() {
    assert_eq!(
        Solution::cherry_pickup(vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]]),
        5
    );
}

#[test]
fn test_cherry_pickup2() {
    assert_eq!(
        Solution::cherry_pickup(vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]]),
        0
    );
}
