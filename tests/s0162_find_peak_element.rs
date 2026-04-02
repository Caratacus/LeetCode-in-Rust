// Tests for Problem 0162: Find Peak Element
// Java reference: src/test/java/g0121_0200/s0162_find_peak_element/SolutionTest.java

use leetcode_in_rust::s0162::find_peak_element::Solution;

#[test]
fn test_find_peak_element() {
    let result = Solution::find_peak_element(vec![1, 2, 3, 1]);
    assert!(result == 2);
}

#[test]
fn test_find_peak_element2() {
    let result = Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]);
    assert!(result == 1 || result == 5);
}

#[test]
fn test_find_peak_element3() {
    let result = Solution::find_peak_element(vec![1]);
    assert!(result == 0);
}
