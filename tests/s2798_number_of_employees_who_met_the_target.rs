// Tests for Problem 2798: Number of Employees Who Met the Target
// Java reference: src/test/java/g2701_2800/s2798_number_of_employees_who_met_the_target/SolutionTest.java

use leetcode_in_rust::s2798::number_of_employees_who_met_the_target::Solution;

#[test]
fn test_number_of_employees_who_met_target() {
    assert_eq!(Solution::number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2), 3);
}

#[test]
fn test_number_of_employees_who_met_target2() {
    assert_eq!(Solution::number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6), 0);
}
