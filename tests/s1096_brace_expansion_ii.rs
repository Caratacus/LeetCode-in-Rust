// Tests for Problem 1096: Brace Expansion II
// Java reference: src/test/java/g1001_1100/s1096_brace_expansion_ii/SolutionTest.java

use leetcode_in_rust::s1096::brace_expansion_ii::Solution;

#[test]
fn test_brace_expansion_ii() {
    assert_eq!(
        Solution::brace_expansion_ii("{a,b}{c,{d,e}}".to_string()),
        vec!["ac".to_string(), "ad".to_string(), "ae".to_string(), "bc".to_string(), "bd".to_string(), "be".to_string()]
    );
}

#[test]
fn test_brace_expansion_ii2() {
    assert_eq!(
        Solution::brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string()),
        vec!["a".to_string(), "ab".to_string(), "ac".to_string(), "z".to_string()]
    );
}
