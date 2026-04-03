// Tests for Problem 0781: Rabbits in Forest
// Java reference: src/test/java/g0701_0800/s0781_rabbits_in_forest/SolutionTest.java

use leetcode_in_rust::s0781::rabbits_in_forest::Solution;

#[test]
fn test_num_rabbits() {
    assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
}

#[test]
fn test_num_rabbits2() {
    assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
}

#[test]
fn test_num_rabbits3() {
    assert_eq!(Solution::num_rabbits(vec![1, 0, 1, 0, 0]), 5);
}
