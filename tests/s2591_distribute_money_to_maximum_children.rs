// Tests for Problem 2591: Distribute Money to Maximum Children
// Java reference: src/test/java/g2501_2600/s2591_distribute_money_to_maximum_children/SolutionTest.java

use leetcode_in_rust::s2591::distribute_money_to_maximum_children::Solution;

#[test]
fn test_dist_money() {
    assert_eq!(Solution::dist_money(20, 3), 1);
}

#[test]
fn test_dist_money2() {
    assert_eq!(Solution::dist_money(16, 2), 2);
}

#[test]
fn test_dist_money3() {
    assert_eq!(Solution::dist_money(1, 2), -1);
}

#[test]
fn test_dist_money4() {
    assert_eq!(Solution::dist_money(2, 1), 0);
}
