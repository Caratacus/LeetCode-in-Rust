// Tests for Problem 2386: Find the K-Sum of an Array
// Java reference: src/test/java/g2301_2400/s2386_find_the_k_sum_of_an_array/SolutionTest.java

use leetcode_in_rust::s2386::find_the_k_sum_of_an_array::Solution;

#[test]
fn test_k_sum() {
    assert_eq!(Solution::k_sum(vec![2, 4, -2], 5), 2_i64);
}

#[test]
fn test_k_sum2() {
    assert_eq!(Solution::k_sum(vec![1, -2, 3, 4, -10, 12], 16), 10_i64);
}

#[test]
fn test_k_sum3() {
    assert_eq!(
        Solution::k_sum(vec![-530219056, 353285209, 493533664], 6),
        -36685392_i64
    );
}
