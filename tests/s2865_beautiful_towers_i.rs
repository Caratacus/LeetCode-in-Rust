// Tests for Problem 2865: Beautiful Towers I
// Java reference: src/test/java/g2801_2900/s2865_beautiful_towers_i/SolutionTest.java

use leetcode_in_rust::s2865::beautiful_towers_i::Solution;

#[test]
fn test_maximum_sum_of_heights() {
    assert_eq!(Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]), 13);
}

#[test]
fn test_maximum_sum_of_heights2() {
    assert_eq!(Solution::maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]), 22);
}

#[test]
fn test_maximum_sum_of_heights3() {
    assert_eq!(Solution::maximum_sum_of_heights(vec![3, 2, 5, 5, 2, 3]), 18);
}
