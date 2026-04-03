// Tests for Problem 1304: Find N Unique Integers Sum up to Zero
// Java reference: src/test/java/g1301_1400/s1304_find_n_unique_integers_sum_up_to_zero/SolutionTest.java

use leetcode_in_rust::s1304::find_n_unique_integers_sum_up_to_zero::Solution;

#[test]
fn test_sum_zero() {
    assert_eq!(Solution::sum_zero(5), vec![-2, -1, 0, 1, 2]);
}

#[test]
fn test_sum_zero2() {
    assert_eq!(Solution::sum_zero(3), vec![-1, 0, 1]);
}

#[test]
fn test_sum_zero3() {
    assert_eq!(Solution::sum_zero(1), vec![0]);
}
