// Tests for Problem 3480: Maximize Subarrays After Removing One Conflicting Pair
// Java reference: src/test/java/g3401_3500/s3480_maximize_subarrays_after_removing_one_conflicting_pair/SolutionTest.java

use leetcode_in_rust::s3480::maximize_subarrays_after_removing_one_conflicting_pair::Solution;

#[test]
fn test_max_subarrays() {
    assert_eq!(Solution::max_subarrays(4, vec![vec![2, 3], vec![1, 4]]), 9i64);
}

#[test]
fn test_max_subarrays2() {
    assert_eq!(Solution::max_subarrays(5, vec![vec![1, 2], vec![2, 5], vec![3, 5]]), 12i64);
}

#[test]
fn test_max_subarrays3() {
    assert_eq!(Solution::max_subarrays(10, vec![vec![10, 5], vec![3, 8]]), 50i64);
}

#[test]
fn test_max_subarrays4() {
    assert_eq!(Solution::max_subarrays(25, vec![vec![9, 7], vec![15, 7], vec![4, 7]]), 216i64);
}
