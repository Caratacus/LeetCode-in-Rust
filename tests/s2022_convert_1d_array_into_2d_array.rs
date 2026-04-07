// Tests for Problem 2022: Convert 1D Array Into 2D Array
// Java reference: src/test/java/g2001_2100/s2022_convert_1d_array_into_2d_array/SolutionTest.java

use leetcode_in_rust::s2022::convert_1d_array_into_2d_array::Solution;

#[test]
fn test_construct2_d_array() {
    assert_eq!(
        Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2),
        vec![vec![1, 2], vec![3, 4]]
    );
}

#[test]
fn test_construct2_d_array2() {
    assert_eq!(
        Solution::construct2_d_array(vec![1, 2, 3], 1, 3),
        vec![vec![1, 2, 3]]
    );
}

#[test]
fn test_construct2_d_array3() {
    assert_eq!(
        Solution::construct2_d_array(vec![1, 2], 1, 1),
        vec![] as Vec<Vec<i32>>
    );
}
