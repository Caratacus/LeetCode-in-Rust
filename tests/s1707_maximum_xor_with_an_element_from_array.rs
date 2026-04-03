// Tests for Problem 1707: Maximum XOR With an Element From Array
// Java reference: src/test/java/g1701_1800/s1707_maximum_xor_with_an_element_from_array/SolutionTest.java

use leetcode_in_rust::s1707::maximum_xor_with_an_element_from_array::Solution;

#[test]
fn test_maximize_xor() {
    assert_eq!(
        Solution::maximize_xor(vec![0, 1, 2, 3, 4], vec![vec![3, 1], vec![1, 3], vec![5, 6]]),
        vec![3, 3, 7]
    );
}

#[test]
fn test_maximize_xor2() {
    assert_eq!(
        Solution::maximize_xor(vec![5, 2, 4, 6, 6, 3], vec![vec![12, 4], vec![8, 1], vec![6, 3]]),
        vec![15, -1, 5]
    );
}
