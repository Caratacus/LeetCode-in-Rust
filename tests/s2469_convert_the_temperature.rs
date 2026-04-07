// Tests for Problem 2469: Convert the Temperature
// Java reference: src/test/java/g2401_2500/s2469_convert_the_temperature/SolutionTest.java

use leetcode_in_rust::s2469::convert_the_temperature::Solution;

#[test]
fn test_convert_temperature() {
    let result = Solution::convert_temperature(36.50);
    assert!((result[0] - 309.65).abs() < 0.01);
    assert!((result[1] - 97.7).abs() < 0.01);
}

#[test]
fn test_convert_temperature2() {
    let result = Solution::convert_temperature(122.11);
    assert!((result[0] - 395.26).abs() < 0.01);
    assert!((result[1] - 251.798).abs() < 0.01);
}
