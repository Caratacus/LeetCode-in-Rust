// Tests for Problem 0838: Push Dominoes
// Java reference: src/test/java/g0801_0900/s0838_push_dominoes/SolutionTest.java

use leetcode_in_rust::s0838::push_dominoes::Solution;

#[test]
fn test_push_dominoes() {
    assert_eq!(Solution::push_dominoes("RR.L".to_string()), "RR.RL".to_string());
}

#[test]
fn test_push_dominoes2() {
    assert_eq!(
        Solution::push_dominoes(".L.R...LR..L..".to_string()),
        "LL.RR.LLRRLL..".to_string()
    );
}
