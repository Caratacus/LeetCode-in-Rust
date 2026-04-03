// Tests for Problem 0697: Degree of an Array
// Java reference: src/test/java/g0601_0700/s0697_degree_of_an_array/SolutionTest.java

use leetcode_in_rust::s0697::degree_of_an_array::Solution;

#[test]
fn test_find_shortest_sub_array() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
}

#[test]
fn test_find_shortest_sub_array2() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]), 6);
}
