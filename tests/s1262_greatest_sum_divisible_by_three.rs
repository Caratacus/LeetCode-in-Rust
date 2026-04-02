// Tests for Problem 1262: Greatest Sum Divisible by Three
// Java reference: src/test/java/g1201_1300/s1262_greatest_sum_divisible_by_three/SolutionTest.java

use leetcode_in_rust::s1262::greatest_sum_divisible_by_three::Solution;

#[test]
fn test_max_sum_div_three() {
    assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
}

#[test]
fn test_max_sum_div_three2() {
    assert_eq!(Solution::max_sum_div_three(vec![4]), 0);
}

#[test]
fn test_max_sum_div_three3() {
    assert_eq!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
}
