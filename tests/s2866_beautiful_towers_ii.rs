// Tests for Problem 2866: Beautiful Towers II
// Java reference: src/test/java/g2801_2900/s2866_beautiful_towers_ii/SolutionTest.java

use leetcode_in_rust::s2866::beautiful_towers_ii::Solution;

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
