// Tests for Problem 2303: Calculate Amount Paid in Taxes
// Java reference: src/test/java/g2301_2400/s2303_calculate_amount_paid_in_taxes/SolutionTest.java

use leetcode_in_rust::s2303::calculate_amount_paid_in_taxes::Solution;

#[test]
fn test_calculate_tax() {
    let result = Solution::calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10);
    assert!((result - 2.65).abs() < 0.00001);
}

#[test]
fn test_calculate_tax2() {
    let result = Solution::calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2);
    assert!((result - 0.25).abs() < 0.00001);
}

#[test]
fn test_calculate_tax3() {
    let result = Solution::calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 0);
    assert!((result - 0.0).abs() < 0.00001);
}
