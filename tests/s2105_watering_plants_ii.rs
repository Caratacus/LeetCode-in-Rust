// Tests for Problem 2105: Watering Plants II
// Java reference: src/test/java/g2101_2200/s2105_watering_plants_ii/SolutionTest.java

use leetcode_in_rust::s2105::watering_plants_ii::Solution;

#[test]
fn test_minimum_refill() {
    assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 5, 5), 1);
}

#[test]
fn test_minimum_refill2() {
    assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 3, 4), 2);
}

#[test]
fn test_minimum_refill3() {
    assert_eq!(Solution::minimum_refill(vec![5], 10, 8), 0);
}

#[test]
fn test_minimum_refill4() {
    assert_eq!(Solution::minimum_refill(vec![1, 2, 4, 4, 5], 6, 5), 2);
}
