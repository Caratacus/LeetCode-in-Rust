// Tests for Problem 2012: Sum of Beauty in the Array
// Java reference: src/test/java/g2001_2100/s2012_sum_of_beauty_in_the_array/SolutionTest.java

use leetcode_in_rust::s2012::sum_of_beauty_in_the_array::Solution;

#[test]
fn test_sum_of_beauties() {
    assert_eq!(Solution::sum_of_beauties(vec![1, 2, 3]), 2);
}

#[test]
fn test_sum_of_beauties2() {
    assert_eq!(Solution::sum_of_beauties(vec![2, 4, 6, 4]), 1);
}

#[test]
fn test_sum_of_beauties3() {
    assert_eq!(Solution::sum_of_beauties(vec![3, 2, 1]), 0);
}
