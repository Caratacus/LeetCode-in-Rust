// Tests for Problem 2440: Create Components With Same Value
// Java reference: src/test/java/g2401_2500/s2440_create_components_with_same_value/SolutionTest.java

use leetcode_in_rust::s2440::create_components_with_same_value::Solution;

#[test]
fn test_component_value() {
    assert_eq!(
        Solution::component_value(
            vec![6, 2, 2, 2, 6],
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]
        ),
        2
    );
}

#[test]
fn test_component_value2() {
    assert_eq!(Solution::component_value(vec![2], vec![]), 0);
}
