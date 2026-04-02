// Tests for Problem 1011: Capacity To Ship Packages Within D Days
// Java reference: src/test/java/g1001_1100/s1011_capacity_to_ship_packages_within_d_days/SolutionTest.java

use leetcode_in_rust::s1011::capacity_to_ship_packages_within_d_days::Solution;

#[test]
fn test_ship_within_days() {
    assert_eq!(
        Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
        15
    );
}

#[test]
fn test_ship_within_days2() {
    assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
}
