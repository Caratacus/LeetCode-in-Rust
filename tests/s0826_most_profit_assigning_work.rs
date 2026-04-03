// Tests for Problem 0826: Most Profit Assigning Work
// Java reference: src/test/java/g0801_0900/s0826_most_profit_assigning_work/SolutionTest.java

use leetcode_in_rust::s0826::most_profit_assigning_work::Solution;

#[test]
fn test_max_profit_assignment() {
    assert_eq!(
        Solution::max_profit_assignment(
            vec![2, 4, 6, 8, 10],
            vec![10, 20, 30, 40, 50],
            vec![4, 5, 6, 7]
        ),
        2
    );
}

#[test]
fn test_max_profit_assignment2() {
    assert_eq!(
        Solution::max_profit_assignment(
            vec![85, 47, 57],
            vec![24, 66, 99],
            vec![40, 25, 80]
        ),
        0
    );
}
