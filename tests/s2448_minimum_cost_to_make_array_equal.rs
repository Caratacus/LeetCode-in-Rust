// Tests for Problem 2448: Minimum Cost to Make Array Equal
// Java reference: src/test/java/g2401_2500/s2448_minimum_cost_to_make_array_equal/SolutionTest.java

use leetcode_in_rust::s2448::minimum_cost_to_make_array_equal::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(
        Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]),
        8
    );
}

#[test]
fn test_min_cost2() {
    assert_eq!(
        Solution::min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3]),
        0
    );
}
