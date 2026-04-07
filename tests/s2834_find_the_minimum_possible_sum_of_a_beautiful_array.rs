// Tests for Problem 2834: Find the Minimum Possible Sum of a Beautiful Array
// Java reference: src/test/java/g2801_2900/s2834_find_the_minimum_possible_sum_of_a_beautiful_array/SolutionTest.java

use leetcode_in_rust::s2834::find_the_minimum_possible_sum_of_a_beautiful_array::Solution;

#[test]
fn test_minimum_possible_sum() {
    assert_eq!(Solution::minimum_possible_sum(2, 3), 4);
}

#[test]
fn test_minimum_possible_sum2() {
    assert_eq!(Solution::minimum_possible_sum(3, 3), 8);
}

#[test]
fn test_minimum_possible_sum3() {
    assert_eq!(Solution::minimum_possible_sum(1, 1), 1);
}
