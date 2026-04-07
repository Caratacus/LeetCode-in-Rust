// Tests for Problem 2148: Count Elements With Strictly Smaller and Greater Elements
// Java reference: src/test/java/g2101_2200/s2148_count_elements_with_strictly_smaller_and_greater_elements/SolutionTest.java

use leetcode_in_rust::s2148::count_elements_with_strictly_smaller_and_greater_elements::Solution;

#[test]
fn test_count_elements() {
    assert_eq!(Solution::count_elements(vec![11, 7, 2, 15]), 2);
}

#[test]
fn test_count_elements2() {
    assert_eq!(Solution::count_elements(vec![-3, 3, 3, 90]), 2);
}

#[test]
fn test_count_elements3() {
    assert_eq!(Solution::count_elements(vec![-71, -71, 93, -71, 40]), 1);
}
