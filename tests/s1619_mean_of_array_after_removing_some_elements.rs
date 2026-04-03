// Tests for Problem 1619: Mean of Array After Removing Some Elements
// Java reference: src/test/java/g1601_1700/s1619_mean_of_array_after_removing_some_elements/SolutionTest.java

use leetcode_in_rust::s1619::mean_of_array_after_removing_some_elements::Solution;

#[test]
fn test_trim_mean() {
    let result = Solution::trim_mean(vec![
        1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3,
    ]);
    assert!((result - 2.0).abs() < 1e-5);
}

#[test]
fn test_trim_mean2() {
    let result = Solution::trim_mean(vec![
        6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0,
    ]);
    assert!((result - 4.0).abs() < 1e-5);
}

#[test]
fn test_trim_mean3() {
    let result = Solution::trim_mean(vec![
        6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 6, 2, 2, 7, 4, 2, 8, 2,
        5, 6, 4, 8, 2, 2, 2, 7, 5, 8, 8, 5, 6, 2, 8, 2, 6, 4, 0, 6, 0, 0, 7, 0, 3, 8, 8, 5, 4, 6,
        2, 2, 4, 7, 1, 7, 3, 7, 0, 8, 5, 0, 6, 4, 4, 8, 1, 7, 6, 6,
    ]);
    assert!((result - 4.77778).abs() < 1e-4);
}
