// Tests for Problem 3357: Minimize the Maximum Adjacent Element Difference
// Java reference: src/test/java/g3301_3400/s3357_minimize_the_maximum_adjacent_element_difference/SolutionTest.java

use leetcode_in_rust::s3357::minimize_the_maximum_adjacent_element_difference::Solution;

#[test]
fn test_min_difference() {
    assert_eq!(Solution::min_difference(vec![1, 2, -1, 10, 8]), 4);
}

#[test]
fn test_min_difference2() {
    assert_eq!(Solution::min_difference(vec![-1, -1, -1]), 0);
}

#[test]
fn test_min_difference3() {
    assert_eq!(Solution::min_difference(vec![-1, 10, -1, 8]), 1);
}

#[test]
fn test_min_difference4() {
    assert_eq!(Solution::min_difference(vec![14, -1, -1, 46]), 11);
}
