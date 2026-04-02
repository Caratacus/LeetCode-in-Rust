// Tests for Problem 0799: Champagne Tower
// Java reference: src/test/java/g0701_0800/s0799_champagne_tower/SolutionTest.java

use leetcode_in_rust::s0799::champagne_tower::Solution;

#[test]
fn test_champagne_tower() {
    let result = Solution::champagne_tower(8, 3, 0);
    assert!((result - 0.125).abs() < 1e-9);
}

#[test]
fn test_champagne_tower2() {
    let result = Solution::champagne_tower(8, 3, 1);
    assert!((result - 0.875).abs() < 1e-9);
}

#[test]
fn test_champagne_tower3() {
    let result = Solution::champagne_tower(8, 3, 2);
    assert!((result - 0.875).abs() < 1e-9);
}

#[test]
fn test_champagne_tower4() {
    let result = Solution::champagne_tower(1000000000, 99, 99);
    assert!((result - 0.0).abs() < 1e-9);
}
