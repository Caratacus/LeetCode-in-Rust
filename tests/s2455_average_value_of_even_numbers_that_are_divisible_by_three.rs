// Tests for Problem 2455: Average Value of Even Numbers That Are Divisible by Three
// Java reference: src/test/java/g2401_2500/s2455_average_value_of_even_numbers_that_are_divisible_by_three/SolutionTest.java

use leetcode_in_rust::s2455::average_value_of_even_numbers_that_are_divisible_by_three::Solution;

#[test]
fn test_average_value() {
    assert_eq!(Solution::average_value(vec![1, 3, 6, 10, 12, 15]), 9);
}

#[test]
fn test_average_value2() {
    assert_eq!(Solution::average_value(vec![1, 2, 4, 7, 10]), 0);
}
