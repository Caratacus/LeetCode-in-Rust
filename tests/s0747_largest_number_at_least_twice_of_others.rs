// Tests for Problem 0747: Largest Number At Least Twice of Others
// Java reference: src/test/java/g0701_0800/s0747_largest_number_at_least_twice_of_others/SolutionTest.java

use leetcode_in_rust::s0747::largest_number_at_least_twice_of_others::Solution;

#[test]
fn test_dominant_index() {
    assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
}

#[test]
fn test_dominant_index2() {
    assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
}

#[test]
fn test_dominant_index3() {
    assert_eq!(Solution::dominant_index(vec![1]), 0);
}
