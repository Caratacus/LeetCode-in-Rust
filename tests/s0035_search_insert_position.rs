// Tests for Problem 0035: Search Insert Position
// Java reference: src/test/java/g0001_0100/s0035_search_insert_position/SolutionTest.java

use leetcode_in_rust::s0035::search_insert_position::Solution;

#[test]
fn test_search_insert() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
}

#[test]
fn test_search_insert2() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
}

#[test]
fn test_search_insert3() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
}

#[test]
fn test_search_insert4() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
}
