// Tests for Problem 2129: Capitalize the Title
// Java reference: src/test/java/g2101_2200/s2129_capitalize_the_title/SolutionTest.java

use leetcode_in_rust::s2129::capitalize_the_title::Solution;

#[test]
fn test_capitalize_title() {
    assert_eq!(
        Solution::capitalize_title("capiTalIze tHe titLe".to_string()),
        "Capitalize The Title"
    );
}

#[test]
fn test_capitalize_title2() {
    assert_eq!(
        Solution::capitalize_title("First leTTeR of EACH Word".to_string()),
        "First Letter of Each Word"
    );
}

#[test]
fn test_capitalize_title3() {
    assert_eq!(
        Solution::capitalize_title("i lOve leetcode".to_string()),
        "i Love Leetcode"
    );
}
