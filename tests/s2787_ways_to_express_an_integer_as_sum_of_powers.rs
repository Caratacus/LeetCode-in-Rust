// Tests for Problem 2787: Ways to Express an Integer as Sum of Powers
// Java reference: src/test/java/g2701_2800/s2787_ways_to_express_an_integer_as_sum_of_powers/SolutionTest.java

use leetcode_in_rust::s2787::ways_to_express_an_integer_as_sum_of_powers::Solution;

#[test]
fn test_number_of_ways() {
    assert_eq!(Solution::number_of_ways(10, 2), 1);
}

#[test]
fn test_number_of_ways2() {
    assert_eq!(Solution::number_of_ways(4, 1), 2);
}
