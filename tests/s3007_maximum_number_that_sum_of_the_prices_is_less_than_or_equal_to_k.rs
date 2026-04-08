// Tests for Problem 3007: Maximum Number That Sum of the Prices Is Less Than or Equal to K
// Java reference: src/test/java/g3001_3100/s3007_maximum_number_that_sum_of_the_prices_is_less_than_or_equal_to_k/SolutionTest.java

use leetcode_in_rust::s3007::maximum_number_that_sum_of_the_prices_is_less_than_or_equal_to_k::Solution;

#[test]
fn test_find_maximum_number() {
    assert_eq!(Solution::find_maximum_number(9, 1), 6);
}

#[test]
fn test_find_maximum_number2() {
    assert_eq!(Solution::find_maximum_number(7, 2), 9);
}
