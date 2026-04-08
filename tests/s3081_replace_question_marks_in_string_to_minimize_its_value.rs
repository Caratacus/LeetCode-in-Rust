// Tests for Problem 3081: Replace Question Marks in String to Minimize Its Value
// Java reference: src/test/java/g3001_3100/s3081_replace_question_marks_in_string_to_minimize_its_value/SolutionTest.java

use leetcode_in_rust::s3081::replace_question_marks_in_string_to_minimize_its_value::Solution;

#[test]
fn test_minimize_string_value() {
    assert_eq!(
        Solution::minimize_string_value("???".to_string()),
        "abc".to_string()
    );
}

#[test]
fn test_minimize_string_value2() {
    assert_eq!(
        Solution::minimize_string_value("a?a?".to_string()),
        "abac".to_string()
    );
}
