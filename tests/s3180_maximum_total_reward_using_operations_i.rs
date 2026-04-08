// Tests for Problem 3180: Maximum Total Reward Using Operations I
// Java reference: src/test/java/g3101_3200/s3180_maximum_total_reward_using_operations_i/SolutionTest.java

use leetcode_in_rust::s3180::maximum_total_reward_using_operations_i::Solution;

#[test]
fn test_max_total_reward() {
    assert_eq!(Solution::max_total_reward(vec![1, 1, 3, 3]), 4);
}

#[test]
fn test_max_total_reward2() {
    assert_eq!(Solution::max_total_reward(vec![1, 6, 4, 3, 2]), 11);
}
