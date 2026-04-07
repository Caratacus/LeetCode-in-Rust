// Tests for Problem 2646: Minimize the Total Price of the Trips
// Java reference: src/test/java/g2601_2700/s2646_minimize_the_total_price_of_the_trips/SolutionTest.java

use leetcode_in_rust::s2646::minimize_the_total_price_of_the_trips::Solution;

#[test]
fn test_minimum_total_price() {
    assert_eq!(
        Solution::minimum_total_price(
            4,
            vec![vec![0, 1], vec![1, 2], vec![1, 3]],
            vec![2, 2, 10, 6],
            vec![vec![0, 3], vec![2, 1], vec![2, 3]]
        ),
        23
    );
}

#[test]
fn test_minimum_total_price2() {
    assert_eq!(
        Solution::minimum_total_price(2, vec![vec![0, 1]], vec![2, 2], vec![vec![0, 0]]),
        1
    );
}
