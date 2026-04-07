// Tests for Problem 2438: Range Product Queries of Powers
// Java reference: src/test/java/g2401_2500/s2438_range_product_queries_of_powers/SolutionTest.java

use leetcode_in_rust::s2438::range_product_queries_of_powers::Solution;

#[test]
fn test_product_queries() {
    assert_eq!(
        Solution::product_queries(15, vec![vec![0, 1], vec![2, 2], vec![0, 3]]),
        vec![2, 4, 64]
    );
}

#[test]
fn test_product_queries2() {
    assert_eq!(Solution::product_queries(2, vec![vec![0, 0]]), vec![2]);
}
