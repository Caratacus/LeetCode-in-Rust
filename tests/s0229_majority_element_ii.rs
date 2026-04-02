// Tests for Problem 0229: Majority Element II
// Java reference: src/test/java/g0201_0300/s0229_majority_element_ii/SolutionTest.java

use leetcode_in_rust::s0229::majority_element_ii::Solution;

#[test]
fn test_majority_element() {
    let mut result = Solution::majority_element(vec![3, 2, 3]);
    result.sort();
    assert_eq!(result, vec![3]);
}

#[test]
fn test_majority_element2() {
    let mut result = Solution::majority_element(vec![1]);
    result.sort();
    assert_eq!(result, vec![1]);
}

#[test]
fn test_majority_element3() {
    let mut result = Solution::majority_element(vec![1, 2]);
    result.sort();
    assert_eq!(result, vec![1, 2]);
}
