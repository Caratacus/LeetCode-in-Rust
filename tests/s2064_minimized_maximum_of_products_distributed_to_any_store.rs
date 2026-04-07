// Tests for Problem 2064: Minimized Maximum of Products Distributed to Any Store
// Java reference: src/test/java/g2001_2100/s2064_minimized_maximum_of_products_distributed_to_any_store/SolutionTest.java

use leetcode_in_rust::s2064::minimized_maximum_of_products_distributed_to_any_store::Solution;

#[test]
fn test_minimized_maximum() {
    assert_eq!(Solution::minimized_maximum(6, vec![11, 6]), 3);
}

#[test]
fn test_minimized_maximum2() {
    assert_eq!(Solution::minimized_maximum(7, vec![15, 10, 10]), 5);
}

#[test]
fn test_minimized_maximum3() {
    assert_eq!(Solution::minimized_maximum(1, vec![100000]), 100000);
}
