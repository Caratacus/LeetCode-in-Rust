// Tests for Problem 0848: Shifting Letters
// Java reference: src/test/java/g0801_0900/s0848_shifting_letters/SolutionTest.java

use leetcode_in_rust::s0848::shifting_letters::Solution;

#[test]
fn test_shifting_letters() {
    assert_eq!(
        Solution::shifting_letters("abc".to_string(), vec![3, 5, 9]),
        "rpl".to_string()
    );
}

#[test]
fn test_shifting_letters2() {
    assert_eq!(
        Solution::shifting_letters("aaa".to_string(), vec![1, 2, 3]),
        "gfd".to_string()
    );
}
