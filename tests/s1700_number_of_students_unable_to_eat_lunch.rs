// Tests for Problem 1700: Number of Students Unable to Eat Lunch
// Java reference: src/test/java/g1601_1700/s1700_number_of_students_unable_to_eat_lunch/SolutionTest.java

use leetcode_in_rust::s1700::number_of_students_unable_to_eat_lunch::Solution;

#[test]
fn test_count_students() {
    assert_eq!(Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0);
}

#[test]
fn test_count_students2() {
    assert_eq!(
        Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
        3
    );
}
