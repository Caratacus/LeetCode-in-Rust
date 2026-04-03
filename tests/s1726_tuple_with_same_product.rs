// Tests for Problem 1726: Tuple with Same Product
// Java reference: src/test/java/g1701_1800/s1726_tuple_with_same_product/SolutionTest.java

use leetcode_in_rust::s1726::tuple_with_same_product::Solution;

#[test]
fn test_tuple_same_product() {
    assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6]), 8);
}

#[test]
fn test_tuple_same_product2() {
    assert_eq!(Solution::tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
}
