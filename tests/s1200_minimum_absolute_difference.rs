// Tests for Problem 1200: Minimum Absolute Difference
// Java reference: src/test/java/g1101_1200/s1200_minimum_absolute_difference/SolutionTest.java

use leetcode_in_rust::s1200::minimum_absolute_difference::Solution;

#[test]
fn test_minimum_abs_difference() {
    let mut result = Solution::minimum_abs_difference(vec![4, 2, 1, 3]);
    result.sort();
    assert_eq!(result, vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
}

#[test]
fn test_minimum_abs_difference2() {
    let mut result = Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]);
    result.sort();
    assert_eq!(result, vec![vec![1, 3]]);
}

#[test]
fn test_minimum_abs_difference3() {
    let mut result = Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]);
    result.sort();
    assert_eq!(result, vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);
}
