// Tests for Problem 0996: Number of Squareful Arrays
// Java reference: src/test/java/g0901_1000/s0996_number_of_squareful_arrays/SolutionTest.java

use leetcode_in_rust::s0996::number_of_squareful_arrays::Solution;

#[test]
fn test_num_squareful_perms() {
    assert_eq!(Solution::num_squareful_perms(vec![1, 17, 8]), 2);
}

#[test]
fn test_num_squareful_perms2() {
    assert_eq!(Solution::num_squareful_perms(vec![2, 2, 2]), 1);
}
