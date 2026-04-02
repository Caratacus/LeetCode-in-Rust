// Tests for Problem 0779: K-th Symbol in Grammar
// Java reference: src/test/java/g0701_0800/s0779_k_th_symbol_in_grammar/SolutionTest.java

use leetcode_in_rust::s0779::k_th_symbol_in_grammar::Solution;

#[test]
fn test_kth_grammar() {
    assert_eq!(Solution::kth_grammar(1, 1), 0);
}

#[test]
fn test_kth_grammar2() {
    assert_eq!(Solution::kth_grammar(2, 1), 0);
}

#[test]
fn test_kth_grammar3() {
    assert_eq!(Solution::kth_grammar(2, 2), 1);
}
