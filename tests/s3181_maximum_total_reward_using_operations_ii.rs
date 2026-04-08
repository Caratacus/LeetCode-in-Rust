// Tests for Problem 3181: Maximum Total Reward Using Operations II
// Java reference: src/test/java/g3101_3200/s3181_maximum_total_reward_using_operations_ii/SolutionTest.java

use leetcode_in_rust::s3181::maximum_total_reward_using_operations_ii::Solution;

#[test]
fn test_max_total_reward() {
    assert_eq!(Solution::max_total_reward(vec![1, 1, 3, 3]), 4);
}

#[test]
fn test_max_total_reward2() {
    assert_eq!(Solution::max_total_reward(vec![1, 6, 4, 3, 2]), 11);
}
