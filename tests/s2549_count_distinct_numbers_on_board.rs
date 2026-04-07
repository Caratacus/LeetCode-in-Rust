// Tests for Problem 2549: Count Distinct Numbers on Board
// Java reference: src/test/java/g2501_2600/s2549_count_distinct_numbers_on_board/SolutionTest.java
use leetcode_in_rust::s2549::count_distinct_numbers_on_board::Solution;

#[test]
fn test_distinct_integers() {
    assert_eq!(Solution::distinct_integers(5), 4);
}
#[test]
fn test_distinct_integers2() {
    assert_eq!(Solution::distinct_integers(3), 2);
}
#[test]
fn test_distinct_integers3() {
    assert_eq!(Solution::distinct_integers(1), 1);
}
#[test]
fn test_distinct_integers4() {
    assert_eq!(Solution::distinct_integers(2), 1);
}
#[test]
fn test_distinct_integers5() {
    assert_eq!(Solution::distinct_integers(1000), 999);
}
