// Tests for Problem 0470: Implement Rand10() Using Rand7()
// Java reference: src/test/java/g0401_0500/s0470_implement_rand10_using_rand7/SolutionTest.java

use leetcode_in_rust::s0470::implement_rand10_using_rand7::Solution;

#[test]
fn test_rand10() {
    let result = Solution::rand10();
    assert!(result >= 1 && result <= 10);
}

#[test]
fn test_rand102() {
    for _ in 0..2 {
        let result = Solution::rand10();
        assert!(result >= 1 && result <= 10);
    }
}

#[test]
fn test_rand103() {
    for _ in 0..3 {
        let result = Solution::rand10();
        assert!(result >= 1 && result <= 10);
    }
}
