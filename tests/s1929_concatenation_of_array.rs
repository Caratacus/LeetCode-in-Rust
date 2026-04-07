// Tests for Problem 1929: Concatenation of Array
// Java reference: src/test/java/g1901_2000/s1929_concatenation_of_array/SolutionTest.java

use leetcode_in_rust::s1929::concatenation_of_array::Solution;

#[test]
fn test_get_concatenation() {
    assert_eq!(
        Solution::get_concatenation(vec![1, 2, 1]),
        vec![1, 2, 1, 1, 2, 1]
    );
}

#[test]
fn test_get_concatenation2() {
    assert_eq!(
        Solution::get_concatenation(vec![1, 3, 2, 1]),
        vec![1, 3, 2, 1, 1, 3, 2, 1]
    );
}
