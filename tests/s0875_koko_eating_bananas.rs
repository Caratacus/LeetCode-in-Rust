// Tests for Problem 0875: Koko Eating Bananas
// Java reference: src/test/java/g0801_0900/s0875_koko_eating_bananas/SolutionTest.java

use leetcode_in_rust::s0875::koko_eating_bananas::Solution;

#[test]
fn test_min_eating_speed() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
}

#[test]
fn test_min_eating_speed2() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
}

#[test]
fn test_min_eating_speed3() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
}
