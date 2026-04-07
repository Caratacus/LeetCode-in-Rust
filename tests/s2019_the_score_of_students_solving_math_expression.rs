// Tests for Problem 2019: The Score of Students Solving Math Expression
// Java reference: src/test/java/g2001_2100/s2019_the_score_of_students_solving_math_expression/SolutionTest.java

use leetcode_in_rust::s2019::the_score_of_students_solving_math_expression::Solution;

#[test]
fn test_score_of_students() {
    assert_eq!(
        Solution::score_of_students(String::from("3+5*2"), vec![13, 0, 10, 13, 13, 16, 16]),
        19
    );
}

#[test]
fn test_score_of_students2() {
    assert_eq!(
        Solution::score_of_students(String::from("6+0*1"), vec![12, 9, 6, 4, 8, 6]),
        10
    );
}

#[test]
fn test_score_of_students3() {
    assert_eq!(
        Solution::score_of_students(String::from("3+5*2"), vec![13, 0, 10, 13, 13, 16, 16]),
        19
    );
}
