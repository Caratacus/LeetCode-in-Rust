// Tests for Problem 3159: Find Occurrences of an Element in an Array
// Java reference: src/test/java/g3101_3200/s3159_find_occurrences_of_an_element_in_an_array/SolutionTest.java

use leetcode_in_rust::s3159::find_occurrences_of_an_element_in_an_array::Solution;
#[test]
fn test_occurrences_of_element() {
    assert_eq!(
        Solution::occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1),
        vec![0, -1, 2, -1]
    );
}
#[test]
fn test_occurrences_of_element2() {
    assert_eq!(Solution::occurrences_of_element(vec![1, 2, 3], vec![10], 5), vec![-1]);
}
