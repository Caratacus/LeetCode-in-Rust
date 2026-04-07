// Tests for Problem 1920: Build Array from Permutation
// Java reference: src/test/java/g1901_2000/s1920_build_array_from_permutation/SolutionTest.java

use leetcode_in_rust::s1920::build_array_from_permutation::Solution;

#[test]
fn test_build_array() {
    assert_eq!(
        Solution::build_array(vec![0, 2, 1, 5, 3, 4]),
        vec![0, 1, 2, 4, 5, 3]
    );
}

#[test]
fn test_build_array2() {
    assert_eq!(
        Solution::build_array(vec![5, 0, 1, 2, 3, 4]),
        vec![4, 5, 0, 1, 2, 3]
    );
}
