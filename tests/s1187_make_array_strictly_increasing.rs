// Tests for Problem 1187: Make Array Strictly Increasing
// Java reference: src/test/java/g1101_1200/s1187_make_array_strictly_increasing/SolutionTest.java

use leetcode_in_rust::s1187::make_array_strictly_increasing::Solution;

#[test]
fn test_make_array_increasing() {
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]),
        1
    );
}

#[test]
fn test_make_array_increasing2() {
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]),
        2
    );
}

#[test]
fn test_make_array_increasing3() {
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]),
        -1
    );
}
