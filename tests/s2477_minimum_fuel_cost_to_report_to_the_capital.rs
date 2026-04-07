// Tests for Problem 2477: Minimum Fuel Cost to Report to the Capital
// Java reference: src/test/java/g2401_2500/s2477_minimum_fuel_cost_to_report_to_the_capital/SolutionTest.java

use leetcode_in_rust::s2477::minimum_fuel_cost_to_report_to_the_capital::Solution;

#[test]
fn test_minimum_fuel_cost() {
    assert_eq!(
        Solution::minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5),
        3
    );
}

#[test]
fn test_minimum_fuel_cost2() {
    assert_eq!(
        Solution::minimum_fuel_cost(
            vec![
                vec![3, 1], vec![3, 2], vec![1, 0], vec![0, 4], vec![0, 5], vec![4, 6]
            ],
            2
        ),
        7
    );
}

#[test]
fn test_minimum_fuel_cost3() {
    assert_eq!(Solution::minimum_fuel_cost(vec![], 1), 0);
}
