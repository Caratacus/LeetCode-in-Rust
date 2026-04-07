// Tests for Problem 2443: Sum of Number and Its Reverse
// Java reference: src/test/java/g2401_2500/s2443_sum_of_number_and_its_reverse/SolutionTest.java

use leetcode_in_rust::s2443::sum_of_number_and_its_reverse::Solution;

#[test]
fn test_sum_of_number_and_reverse() {
    assert_eq!(Solution::sum_of_number_and_reverse(443), true);
}

#[test]
fn test_sum_of_number_and_reverse2() {
    assert_eq!(Solution::sum_of_number_and_reverse(63), false);
}

#[test]
fn test_sum_of_number_and_reverse3() {
    assert_eq!(Solution::sum_of_number_and_reverse(181), true);
}
