// Tests for Problem 0983: Minimum Cost For Tickets
// Java reference: src/test/java/g0901_1000/s0983_minimum_cost_for_tickets/SolutionTest.java

use leetcode_in_rust::s0983::minimum_cost_for_tickets::Solution;

#[test]
fn test_mincost_tickets() {
    assert_eq!(
        Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
        11
    );
}

#[test]
fn test_mincost_tickets2() {
    assert_eq!(
        Solution::mincost_tickets(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31],
            vec![2, 7, 15]
        ),
        17
    );
}
