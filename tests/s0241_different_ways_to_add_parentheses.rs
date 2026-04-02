// Tests for Problem 0241: Different Ways to Add Parentheses
// Java reference: src/test/java/g0201_0300/s0241_different_ways_to_add_parentheses/SolutionTest.java

use leetcode_in_rust::s0241::different_ways_to_add_parentheses::Solution;

#[test]
fn test_diff_ways_to_compute() {
    let mut result = Solution::diff_ways_to_compute("2-1-1".to_string());
    result.sort();
    let mut expected = vec![0, 2];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_diff_ways_to_compute2() {
    let mut result = Solution::diff_ways_to_compute("2*3-4*5".to_string());
    result.sort();
    let mut expected = vec![-34, -14, -10, -10, 10];
    expected.sort();
    assert_eq!(result, expected);
}
