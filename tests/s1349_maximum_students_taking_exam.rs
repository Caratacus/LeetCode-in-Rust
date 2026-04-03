// Tests for Problem 1349: Maximum Students Taking Exam
// Java reference: src/test/java/g1301_1400/s1349_maximum_students_taking_exam/SolutionTest.java

use leetcode_in_rust::s1349::maximum_students_taking_exam::Solution;

#[test]
fn test_max_students() {
    let result = Solution::max_students(vec![
        vec!['#', '.', '#', '#', '.', '#'],
        vec!['.', '#', '#', '#', '#', '.'],
        vec!['#', '.', '#', '#', '.', '#'],
    ]);
    assert_eq!(result, 4);
}

#[test]
fn test_max_students2() {
    let result = Solution::max_students(vec![vec!['.', '#'], vec!['#', '#'], vec!['#', '.'], vec!['#', '#'], vec!['.', '#']]);
    assert_eq!(result, 3);
}

#[test]
fn test_max_students3() {
    let result = Solution::max_students(vec![
        vec!['#', '.', '.', '.', '#'],
        vec!['.', '#', '.', '#', '.'],
        vec!['.', '.', '#', '.', '.'],
        vec!['.', '#', '.', '#', '.'],
        vec!['#', '.', '.', '.', '#'],
    ]);
    assert_eq!(result, 10);
}
