// Tests for Problem 1547: Minimum Cost to Cut a Stick
// Java reference: src/test/java/g1501_1600/s1547_minimum_cost_to_cut_a_stick/SolutionTest.java

use leetcode_in_rust::s1547::minimum_cost_to_cut_a_stick::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(7, vec![1, 3, 4, 5]), 16);
}

#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(9, vec![5, 6, 1, 4, 2]), 22);
}
