// Tests for Problem 2906: Construct Product Matrix
// Java reference: src/test/java/g2901_3000/s2906_construct_product_matrix/SolutionTest.java

use leetcode_in_rust::s2906::construct_product_matrix::Solution;

#[test]
fn test_construct_product_matrix() {
    assert_eq!(
        Solution::construct_product_matrix(vec![vec![1, 2], vec![3, 4]]),
        vec![vec![24, 12], vec![8, 6]]
    );
}
