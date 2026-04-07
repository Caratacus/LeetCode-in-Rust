// Tests for Problem 2295: Replace Elements in an Array
// Java reference: src/test/java/g2201_2300/s2295_replace_elements_in_an_array/SolutionTest.java

use leetcode_in_rust::s2295::replace_elements_in_an_array::Solution;

#[test]
fn test_array_change() {
    assert_eq!(
        Solution::array_change(vec![1, 2, 4, 6], vec![vec![1, 3], vec![4, 7], vec![6, 1]]),
        vec![3, 2, 7, 1]
    );
}

#[test]
fn test_array_change2() {
    assert_eq!(
        Solution::array_change(vec![1, 2], vec![vec![1, 3], vec![2, 1], vec![3, 2]]),
        vec![2, 1]
    );
}
