// Tests for Problem 1553: Minimum Number of Days to Eat N Oranges
// Java reference: src/test/java/g1501_1600/s1553_minimum_number_of_days_to_eat_n_oranges/SolutionTest.java

use leetcode_in_rust::s1553::minimum_number_of_days_to_eat_n_oranges::Solution;

#[test]
fn test_min_days() {
    assert_eq!(Solution::min_days(10), 4);
}

#[test]
fn test_min_days2() {
    assert_eq!(Solution::min_days(6), 3);
}
