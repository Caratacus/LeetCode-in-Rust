// Tests for Problem 1630: Arithmetic Subarrays
// Java reference: src/test/java/g1601_1700/s1630_arithmetic_subarrays/SolutionTest.java

use leetcode_in_rust::s1630::arithmetic_subarrays::Solution;

#[test]
fn test_check_arithmetic_subarrays() {
    assert_eq!(Solution::check_arithmetic_subarrays(vec![4, 6, 5, 9, 3, 7], vec![0, 0, 2], vec![2, 3, 5]), vec![true, false, true]);
}

#[test]
fn test_check_arithmetic_subarrays2() {
    assert_eq!(
        Solution::check_arithmetic_subarrays(
            vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
            vec![0, 1, 6, 4, 8, 7],
            vec![4, 4, 9, 7, 9, 10]
        ),
        vec![false, true, false, false, true, true]
    );
}
