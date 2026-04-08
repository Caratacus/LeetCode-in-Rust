// Tests for Problem 3295: Report Spam Message
// Java reference: src/test/java/g3201_3300/s3295_report_spam_message/SolutionTest.java

use leetcode_in_rust::s3295::report_spam_message::Solution;

#[test]
fn test_report_spam() {
    assert_eq!(
        Solution::report_spam(
            vec!["hello".to_string(), "world".to_string(), "leetcode".to_string()],
            vec!["world".to_string(), "hello".to_string()]
        ),
        true
    );
}

#[test]
fn test_report_spam2() {
    assert_eq!(
        Solution::report_spam(
            vec!["hello".to_string(), "programming".to_string(), "fun".to_string()],
            vec!["world".to_string(), "programming".to_string(), "leetcode".to_string()]
        ),
        false
    );
}
