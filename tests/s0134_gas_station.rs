// Tests for Problem 0134: Gas Station
// Java reference: src/test/java/g0121_0200/s0134_gas_station/SolutionTest.java

use leetcode_in_rust::s0134::gas_station::Solution;

#[test]
fn test_can_complete_circuit() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
}

#[test]
fn test_can_complete_circuit2() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
}
