// Tests for Problem 2611: Mice and Cheese
// Java reference: src/test/java/g2601_2700/s2611_mice_and_cheese/SolutionTest.java

use leetcode_in_rust::s2611::mice_and_cheese::Solution;

#[test]
fn test_mice_and_cheese() {
    assert_eq!(
        Solution::mice_and_cheese(vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2),
        15
    );
}

#[test]
fn test_mice_and_cheese2() {
    assert_eq!(Solution::mice_and_cheese(vec![1, 1], vec![1, 1], 2), 2);
}
