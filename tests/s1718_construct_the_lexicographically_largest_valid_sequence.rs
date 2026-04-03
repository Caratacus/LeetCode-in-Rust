// Tests for Problem 1718: Construct the Lexicographically Largest Valid Sequence
// Java reference: src/test/java/g1701_1800/s1718_construct_the_lexicographically_largest_valid_sequence/SolutionTest.java

use leetcode_in_rust::s1718::construct_the_lexicographically_largest_valid_sequence::Solution;

#[test]
fn test_construct_distanced_sequence() {
    assert_eq!(
        Solution::construct_distanced_sequence(3),
        vec![3, 1, 2, 3, 2]
    );
}

#[test]
fn test_construct_distanced_sequence2() {
    assert_eq!(
        Solution::construct_distanced_sequence(5),
        vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
    );
}
