// Tests for Problem 1122: Relative Sort Array
// Java reference: src/test/java/g1101_1200/s1122_relative_sort_array/SolutionTest.java

use leetcode_in_rust::s1122::relative_sort_array::Solution;

#[test]
fn test_relative_sort_array() {
    assert_eq!(
        Solution::relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    );
}

#[test]
fn test_relative_sort_array2() {
    assert_eq!(
        Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
        vec![22, 28, 8, 6, 17, 44]
    );
}
