// Tests for Problem 2499: Minimum Total Cost to Make Arrays Unequal
// Java reference: src/test/java/g2401_2500/s2499_minimum_total_cost_to_make_arrays_unequal/SolutionTest.java

use leetcode_in_rust::s2499::minimum_total_cost_to_make_arrays_unequal::Solution;

#[test]
fn test_minimum_total_cost() {
    assert_eq!(
        Solution::minimum_total_cost(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
        10
    );
}

#[test]
fn test_minimum_total_cost2() {
    assert_eq!(
        Solution::minimum_total_cost(vec![2, 2, 2, 1, 3], vec![1, 2, 2, 3, 3]),
        10
    );
}

#[test]
fn test_minimum_total_cost3() {
    assert_eq!(
        Solution::minimum_total_cost(vec![1, 2, 2], vec![1, 2, 2]),
        -1
    );
}
