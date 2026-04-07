// Tests for Problem 2538: Difference Between Maximum and Minimum Price Sum
// Java reference: src/test/java/g2501_2600/s2538_difference_between_maximum_and_minimum_price_sum/SolutionTest.java

use leetcode_in_rust::s2538::difference_between_maximum_and_minimum_price_sum::Solution;

#[test]
fn test_max_output() {
    assert_eq!(
        Solution::max_output(6, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]], vec![9, 8, 7, 6, 10, 5]),
        24
    );
}

#[test]
fn test_max_output2() {
    assert_eq!(
        Solution::max_output(3, vec![vec![0, 1], vec![1, 2]], vec![1, 1, 1]),
        2
    );
}
