// Tests for Problem 3145: Find Products of Elements of Big Array
// Java reference: src/test/java/g3101_3200/s3145_find_products_of_elements_of_big_array/SolutionTest.java

use leetcode_in_rust::s3145::find_products_of_elements_of_big_array::Solution;
#[test]
fn test_find_products_of_elements() {
    assert_eq!(Solution::find_products_of_elements(vec![vec![1, 3, 7]]), vec![4]);
}
#[test]
fn test_find_products_of_elements2() {
    assert_eq!(
        Solution::find_products_of_elements(vec![vec![2, 5, 3], vec![7, 7, 4]]),
        vec![2, 2]
    );
}
