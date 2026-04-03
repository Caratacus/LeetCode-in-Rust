// Tests for Problem 0808: Soup Servings
// Java reference: src/test/java/g0701_0800/s0808_soup_servings/SolutionTest.java

use leetcode_in_rust::s0808::soup_servings::Solution;

#[test]
fn test_soup_servings() {
    let result = Solution::soup_servings(50);
    assert!((result - 0.625).abs() < 0.00001);
}

#[test]
fn test_soup_servings2() {
    let result = Solution::soup_servings(100);
    assert!((result - 0.71875).abs() < 0.00001);
}
