// Tests for Problem 1991: Find the Middle Index in Array
// Java reference: src/test/java/g1901_2000/s1991_find_the_middle_index_in_array/SolutionTest.java

use leetcode_in_rust::s1991::find_the_middle_index_in_array::Solution;

#[test]
fn test_find_middle_index() {
    assert_eq!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]), 3);
}

#[test]
fn test_find_middle_index2() {
    assert_eq!(Solution::find_middle_index(vec![1, -1, 4]), 2);
}

#[test]
fn test_find_middle_index3() {
    assert_eq!(Solution::find_middle_index(vec![2, 5]), -1);
}
