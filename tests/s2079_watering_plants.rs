// Tests for Problem 2079: Watering Plants
// Java reference: src/test/java/g2001_2100/s2079_watering_plants/SolutionTest.java

use leetcode_in_rust::s2079::watering_plants::Solution;

#[test]
fn test_watering_plants() {
    assert_eq!(Solution::watering_plants(vec![2, 2, 3, 3], 5), 14);
}

#[test]
fn test_watering_plants2() {
    assert_eq!(Solution::watering_plants(vec![1, 1, 1, 4, 2, 3], 4), 30);
}

#[test]
fn test_watering_plants3() {
    assert_eq!(Solution::watering_plants(vec![7, 7, 7, 7, 7, 7, 7], 8), 49);
}
