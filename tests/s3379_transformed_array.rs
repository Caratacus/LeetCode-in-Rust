// Tests for Problem 3379: Transformed Array
// Java reference: src/test/java/g3301_3400/s3379_transformed_array/SolutionTest.java

use leetcode_in_rust::s3379::transformed_array::Solution;

#[test]
fn test_construct_transformed_array() {
    assert_eq!(
        Solution::construct_transformed_array(vec![3, -2, 1, 1]),
        vec![1, 1, 1, 3]
    );
}

#[test]
fn test_construct_transformed_array2() {
    assert_eq!(
        Solution::construct_transformed_array(vec![-1, 4, -1]),
        vec![-1, -1, 4]
    );
}
