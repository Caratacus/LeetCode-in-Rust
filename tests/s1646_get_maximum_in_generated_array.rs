// Tests for Problem 1646: Get Maximum in Generated Array
// Java reference: src/test/java/g1601_1700/s1646_get_maximum_in_generated_array/SolutionTest.java

use leetcode_in_rust::s1646::get_maximum_in_generated_array::Solution;

#[test]
fn test_get_maximum_generated() {
    assert_eq!(Solution::get_maximum_generated(7), 3);
}

#[test]
fn test_get_maximum_generated2() {
    assert_eq!(Solution::get_maximum_generated(2), 1);
}

#[test]
fn test_get_maximum_generated3() {
    assert_eq!(Solution::get_maximum_generated(3), 2);
}
