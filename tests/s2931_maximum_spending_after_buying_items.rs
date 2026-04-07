// Tests for Problem 2931: Maximum Spending After Buying Items
// Java reference: src/test/java/g2901_3000/s2931_maximum_spending_after_buying_items/SolutionTest.java

use leetcode_in_rust::s2931::maximum_spending_after_buying_items::Solution;

#[test]
fn test_max_spending() {
    assert_eq!(
        Solution::max_spending(vec![vec![8, 5, 2], vec![6, 4, 1], vec![9, 7, 3]]),
        285
    );
}

#[test]
fn test_max_spending2() {
    assert_eq!(
        Solution::max_spending(vec![vec![10, 8, 6, 4, 2], vec![9, 7, 5, 3, 2]]),
        386
    );
}
