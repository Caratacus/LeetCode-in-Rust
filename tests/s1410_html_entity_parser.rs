// Tests for Problem 1410: HTML Entity Parser
// Java reference: src/test/java/g1401_1500/s1410_html_entity_parser/SolutionTest.java

use leetcode_in_rust::s1410::html_entity_parser::Solution;

#[test]
fn test_entity_parser() {
    assert_eq!(
        Solution::entity_parser("&amp; is an HTML entity but &ambassador; is not.".to_string()),
        "& is an HTML entity but &ambassador; is not."
    );
}

#[test]
fn test_entity_parser2() {
    assert_eq!(
        Solution::entity_parser("and I quote: &quot;...&quot;".to_string()),
        "and I quote: \"...\""
    );
}

#[test]
fn test_entity_parser3() {
    assert_eq!(
        Solution::entity_parser("&frasl;&apos;&gt;&lt;&lt".to_string()),
        "/'><&lt"
    );
}
