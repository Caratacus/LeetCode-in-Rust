// Tests for Problem 1801: Number of Orders in the Backlog
// Java reference: src/test/java/g1801_1900/s1801_number_of_orders_in_the_backlog/SolutionTest.java

use leetcode_in_rust::s1801::number_of_orders_in_the_backlog::Solution;

#[test]
fn test_get_number_of_backlog_orders() {
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![10, 5, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0]
        ]),
        6
    );
}

#[test]
fn test_get_number_of_backlog_orders2() {
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![7, 1000000000, 1],
            vec![15, 3, 0],
            vec![5, 999999995, 0],
            vec![5, 1, 1]
        ]),
        999999984
    );
}
