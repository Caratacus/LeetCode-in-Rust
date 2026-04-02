// Tests for Problem 0354: Russian Doll Envelopes
// Java reference: src/test/java/g0301_0400/s0354_russian_doll_envelopes/SolutionTest.java

use leetcode_in_rust::s0354::russian_doll_envelopes::Solution;

#[test]
fn test_max_envelopes() {
    assert_eq!(Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]), 3);
}

#[test]
fn test_max_envelopes2() {
    assert_eq!(Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]]), 1);
}
