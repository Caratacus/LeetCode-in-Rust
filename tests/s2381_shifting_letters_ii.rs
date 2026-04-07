// Tests for Problem 2381: Shifting Letters II
// Java reference: src/test/java/g2301_2400/s2381_shifting_letters_ii/SolutionTest.java

use leetcode_in_rust::s2381::shifting_letters_ii::Solution;

#[test]
fn test_shifting_letters() {
    let shifts = vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]];
    assert_eq!(Solution::shifting_letters("abc".to_string(), shifts), "ace");
}

#[test]
fn test_shifting_letters2() {
    let shifts = vec![vec![0, 0, 0], vec![1, 1, 1]];
    assert_eq!(Solution::shifting_letters("dztz".to_string(), shifts), "catz");
}
