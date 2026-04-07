// Tests for Problem 2100: Find Good Days to Rob the Bank
// Java reference: src/test/java/g2001_2100/s2100_find_good_days_to_rob_the_bank/SolutionTest.java

use leetcode_in_rust::s2100::find_good_days_to_rob_the_bank::Solution;

#[test]
fn test_good_days_to_rob_bank() {
    assert_eq!(Solution::good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2), vec![2, 3]);
}

#[test]
fn test_good_days_to_rob_bank2() {
    assert_eq!(Solution::good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_good_days_to_rob_bank3() {
    assert_eq!(Solution::good_days_to_rob_bank(vec![1, 2, 3, 4, 5, 6], 2), vec![]);
}
