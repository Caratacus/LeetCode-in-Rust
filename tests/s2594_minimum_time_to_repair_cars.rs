// Tests for Problem 2594: Minimum Time to Repair Cars
// Java reference: src/test/java/g2501_2600/s2594_minimum_time_to_repair_cars/SolutionTest.java

use leetcode_in_rust::s2594::minimum_time_to_repair_cars::Solution;

#[test]
fn test_repair_cars() {
    assert_eq!(Solution::repair_cars(vec![4, 2, 3, 1], 10), 16);
}

#[test]
fn test_repair_cars2() {
    assert_eq!(Solution::repair_cars(vec![5, 1, 8], 6), 16);
}
