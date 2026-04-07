// Tests for Problem 1894: Find the Student that Will Replace the Chalk
// Java reference: src/test/java/g1801_1900/s1894_find_the_student_that_will_replace_the_chalk/SolutionTest.java

use leetcode_in_rust::s1894::find_the_student_that_will_replace_the_chalk::Solution;

#[test]
fn test_chalk_replacer() {
    assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
}

#[test]
fn test_chalk_replacer2() {
    assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
}
