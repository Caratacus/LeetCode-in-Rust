// Tests for Problem 1491: Average Salary Excluding the Minimum and Maximum Salary
// Java reference: src/test/java/g1401_1500/s1491_average_salary_excluding_the_minimum_and_maximum_salary/SolutionTest.java

use leetcode_in_rust::s1491::average_salary_excluding_the_minimum_and_maximum_salary::Solution;

#[test]
fn test_average() {
    let result = Solution::average(vec![4000, 3000, 1000, 2000]);
    assert!((result - 2500.0).abs() < 1e-5);
}

#[test]
fn test_average2() {
    let result = Solution::average(vec![1000, 2000, 3000]);
    assert!((result - 2000.0).abs() < 1e-5);
}
