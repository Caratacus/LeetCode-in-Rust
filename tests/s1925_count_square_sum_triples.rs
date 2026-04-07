// Tests for Problem 1925: Count Square Sum Triples
// Java reference: src/test/java/g1901_2000/s1925_count_square_sum_triples/SolutionTest.java

use leetcode_in_rust::s1925::count_square_sum_triples::Solution;

#[test]
fn test_count_triples() {
    assert_eq!(Solution::count_triples(5), 2);
}

#[test]
fn test_count_triples2() {
    assert_eq!(Solution::count_triples(10), 4);
}
