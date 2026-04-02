// Tests for Problem 0982: Triples with Bitwise AND Equal to Zero
// Java reference: src/test/java/g0901_1000/s0982_triples_with_bitwise_and_equal_to_zero/SolutionTest.java

use leetcode_in_rust::s0982::triples_with_bitwise_and_equal_to_zero::Solution;

#[test]
fn test_count_triplets() {
    assert_eq!(Solution::count_triplets(vec![2, 1, 3]), 12);
}

#[test]
fn test_count_triplets2() {
    assert_eq!(Solution::count_triplets(vec![0, 0, 0]), 27);
}
