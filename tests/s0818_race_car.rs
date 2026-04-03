// Tests for Problem 0818: Race Car
// Java reference: src/test/java/g0701_0800/s0818_race_car/SolutionTest.java

use leetcode_in_rust::s0818::race_car::Solution;

#[test]
fn test_race_car() {
    assert_eq!(Solution::racecar(3), 2);
}

#[test]
fn test_race_car2() {
    assert_eq!(Solution::racecar(6), 5);
}
