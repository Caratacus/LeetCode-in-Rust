// Tests for Problem 1807: Evaluate the Bracket Pairs of a String
// Java reference: src/test/java/g1801_1900/s1807_evaluate_the_bracket_pairs_of_a_string/SolutionTest.java

use std::collections::HashMap;
use leetcode_in_rust::s1807::evaluate_the_bracket_pairs_of_a_string::Solution;

#[test]
fn test_evaluate() {
    let mut knowledge = HashMap::new();
    knowledge.insert("name".to_string(), "bob".to_string());
    knowledge.insert("age".to_string(), "two".to_string());
    assert_eq!(
        Solution::evaluate("(name)is(age)yearsold".to_string(), knowledge),
        "bobistwoyearsold".to_string()
    );
}

#[test]
fn test_evaluate2() {
    let mut knowledge = HashMap::new();
    knowledge.insert("a".to_string(), "b".to_string());
    assert_eq!(
        Solution::evaluate("hi(name)".to_string(), knowledge),
        "hi?".to_string()
    );
}

#[test]
fn test_evaluate3() {
    let mut knowledge = HashMap::new();
    knowledge.insert("a".to_string(), "yes".to_string());
    knowledge.insert("age".to_string(), "two".to_string());
    assert_eq!(
        Solution::evaluate("(a)(a)(a)aaa".to_string(), knowledge),
        "yesyesyesaaa".to_string()
    );
}
