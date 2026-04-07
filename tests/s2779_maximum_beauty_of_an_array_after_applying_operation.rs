// Tests for Problem 2779: Maximum Beauty of an Array After Applying Operation
// Java reference: src/test/java/g2701_2800/s2779_maximum_beauty_of_an_array_after_applying_operation/SolutionTest.java

use leetcode_in_rust::s2779::maximum_beauty_of_an_array_after_applying_operation::Solution;

#[test]
fn test_maximum_beauty() {
    assert_eq!(Solution::maximum_beauty(vec![4, 6, 1, 2], 2), 3);
}

#[test]
fn test_maximum_beauty2() {
    assert_eq!(Solution::maximum_beauty(vec![1, 1, 1, 1], 10), 4);
}
