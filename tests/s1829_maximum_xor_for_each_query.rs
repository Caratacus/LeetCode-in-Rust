// Tests for Problem 1829: Maximum XOR for Each Query
// Java reference: src/test/java/g1801_1900/s1829_maximum_xor_for_each_query/SolutionTest.java

use leetcode_in_rust::s1829::maximum_xor_for_each_query::Solution;

#[test]
fn test_get_maximum_xor() {
    assert_eq!(
        Solution::get_maximum_xor(vec![0, 1, 1, 3], 2),
        vec![0, 3, 2, 3]
    );
}

#[test]
fn test_get_maximum_xor2() {
    assert_eq!(
        Solution::get_maximum_xor(vec![2, 3, 4, 7], 3),
        vec![5, 2, 6, 5]
    );
}

#[test]
fn test_get_maximum_xor3() {
    assert_eq!(
        Solution::get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3),
        vec![4, 3, 6, 4, 6, 7]
    );
}
