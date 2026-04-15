// Tests for Problem 3418: Maximum Amount of Money Robot Can Earn
// Java reference: src/test/java/g3401_3500/s3418_maximum_amount_of_money_robot_can_earn/SolutionTest.java

use leetcode_in_rust::s3418::maximum_amount_of_money_robot_can_earn::Solution;

#[test]
fn test_maximum_amount() {
    assert_eq!(Solution::maximum_amount(vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]]), 8);
}

#[test]
fn test_maximum_amount2() {
    assert_eq!(Solution::maximum_amount(vec![vec![10, 10, 10], vec![10, 10, 10]]), 40);
}
