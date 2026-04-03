// Tests for Problem 0722: Remove Comments
// Java reference: src/test/java/g0701_0800/s0722_remove_comments/SolutionTest.java

use leetcode_in_rust::s0722::remove_comments::Solution;

#[test]
fn test_remove_comments() {
    let input = vec![
        "/*Test program */".to_string(),
        "int main()".to_string(),
        "{ ".to_string(),
        "  // variable declaration ".to_string(),
        "int a, b, c;".to_string(),
        "/* This is a test".to_string(),
        "   multiline  ".to_string(),
        "   comment for ".to_string(),
        "   testing */".to_string(),
        "a = b + c;".to_string(),
        "}".to_string(),
    ];
    let expected = vec![
        "int main()".to_string(),
        "{ ".to_string(),
        "  ".to_string(),
        "int a, b, c;".to_string(),
        "a = b + c;".to_string(),
        "}".to_string(),
    ];
    assert_eq!(Solution::remove_comments(input), expected);
}

#[test]
fn test_remove_comments2() {
    let input = vec![
        "a/*comment".to_string(),
        "line".to_string(),
        "more_comment*/b".to_string(),
    ];
    let expected = vec!["ab".to_string()];
    assert_eq!(Solution::remove_comments(input), expected);
}
