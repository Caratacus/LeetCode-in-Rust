// Tests for Problem 1488: Avoid Flood in The City
// Java reference: src/test/java/g1401_1500/s1488_avoid_flood_in_the_city/SolutionTest.java

use leetcode_in_rust::s1488::avoid_flood_in_the_city::Solution;

#[test]
fn test_avoid_flood() {
    assert_eq!(Solution::avoid_flood(vec![1, 2, 3, 4]), vec![-1, -1, -1, -1]);
}

#[test]
fn test_avoid_flood2() {
    assert_eq!(Solution::avoid_flood(vec![1, 2, 0, 0, 2, 1]), vec![-1, -1, 2, 1, -1, -1]);
}

#[test]
fn test_avoid_flood3() {
    assert_eq!(Solution::avoid_flood(vec![1, 2, 0, 1, 2]), vec![]);
}
