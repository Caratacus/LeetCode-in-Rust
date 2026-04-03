// Tests for Problem 1376: Time Needed to Inform All Employees
// Java reference: src/test/java/g1301_1400/s1376_time_needed_to_inform_all_employees/SolutionTest.java

use leetcode_in_rust::s1376::time_needed_to_inform_all_employees::Solution;

#[test]
fn test_num_of_minutes() {
    assert_eq!(Solution::num_of_minutes(1, 0, vec![-1], vec![0]), 0);
}

#[test]
fn test_num_of_minutes2() {
    assert_eq!(
        Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]),
        1
    );
}
