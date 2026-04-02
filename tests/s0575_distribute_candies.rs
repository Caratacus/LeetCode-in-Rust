// Tests for Problem 0575: Distribute Candies
// Java reference: src/test/java/g0501_0600/s0575_distribute_candies/SolutionTest.java

use leetcode_in_rust::s0575::distribute_candies::Solution;

#[test]
fn test_distribute_candies() {
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
}

#[test]
fn test_distribute_candies2() {
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
}

#[test]
fn test_distribute_candies3() {
    assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
}
