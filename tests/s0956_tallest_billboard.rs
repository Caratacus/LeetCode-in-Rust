// Tests for Problem 0956: Tallest Billboard
// Java reference: src/test/java/g0901_1000/s0956_tallest_billboard/SolutionTest.java

use leetcode_in_rust::s0956::tallest_billboard::Solution;

#[test]
fn test_tallest_billboard() {
    let result = Solution::tallest_billboard(vec![1, 2, 3, 6]);
    assert_eq!(result, 6);
}

#[test]
fn test_tallest_billboard2() {
    let result = Solution::tallest_billboard(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(result, 10);
}

#[test]
fn test_tallest_billboard3() {
    let result = Solution::tallest_billboard(vec![1, 2]);
    assert_eq!(result, 0);
}
