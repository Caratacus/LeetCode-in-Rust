// Tests for Problem 2939: Maximum XOR Product
// Java reference: src/test/java/g2901_3000/s2939_maximum_xor_product/SolutionTest.java

use leetcode_in_rust::s2939::maximum_xor_product::Solution;

#[test]
fn test_maximum_xor_product() {
    assert_eq!(Solution::maximum_xor_product(12, 5, 4), 98);
}

#[test]
fn test_maximum_xor_product2() {
    assert_eq!(Solution::maximum_xor_product(6, 7, 5), 930);
}

#[test]
fn test_maximum_xor_product3() {
    assert_eq!(Solution::maximum_xor_product(1, 6, 3), 12);
}
