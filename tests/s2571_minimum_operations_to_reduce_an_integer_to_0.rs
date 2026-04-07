// Tests for Problem 2571: Minimum Operations to Reduce an Integer to 0
// Java reference: src/test/java/g2501_2600/s2571_minimum_operations_to_reduce_an_integer_to_0/SolutionTest.java

use leetcode_in_rust::s2571::minimum_operations_to_reduce_an_integer_to_0::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(39), 3);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(54), 3);
}
