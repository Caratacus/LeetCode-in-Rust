// Tests for Problem 0787: Cheapest Flights Within K Stops
// Java reference: src/test/java/g0701_0800/s0787_cheapest_flights_within_k_stops/SolutionTest.java

use leetcode_in_rust::s0787::cheapest_flights_within_k_stops::Solution;

#[test]
fn test_find_cheapest_price() {
    assert_eq!(
        Solution::find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            1
        ),
        200
    );
}

#[test]
fn test_find_cheapest_price2() {
    assert_eq!(
        Solution::find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            0
        ),
        500
    );
}
