// Tests for Problem 0476: Number Complement
// Java reference: src/test/java/g0401_0500/s0476_number_complement/SolutionTest.java

use leetcode_in_rust::s0476::number_complement::Solution;

#[test]
fn test_find_complement() {
    assert_eq!(Solution::find_complement(5), 2);
}

#[test]
fn test_find_complement2() {
    assert_eq!(Solution::find_complement(1), 0);
}
