// Tests for Problem 3287: Find the Maximum Sequence Value of Array
// Java reference: src/test/java/g3201_3300/s3287_find_the_maximum_sequence_value_of_array/SolutionTest.java

use leetcode_in_rust::s3287::find_the_maximum_sequence_value_of_array::Solution;

#[test]
fn test_max_value() {
    assert_eq!(Solution::max_value(vec![2, 6, 7], 1), 5);
}

#[test]
fn test_max_value2() {
    assert_eq!(Solution::max_value(vec![4, 2, 5, 6, 7], 2), 2);
}
