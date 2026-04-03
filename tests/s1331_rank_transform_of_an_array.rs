// Tests for Problem 1331: Rank Transform of an Array
// Java reference: src/test/java/g1301_1400/s1331_rank_transform_of_an_array/SolutionTest.java

use leetcode_in_rust::s1331::rank_transform_of_an_array::Solution;

#[test]
fn test_array_rank_transform() {
    assert_eq!(Solution::array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
}

#[test]
fn test_array_rank_transform2() {
    assert_eq!(Solution::array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
}

#[test]
fn test_array_rank_transform3() {
    assert_eq!(
        Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
        vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
    );
}
