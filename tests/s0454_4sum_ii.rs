// Tests for Problem 0454: 4Sum II
// Java reference: src/test/java/g0401_0500/s0454_4sum_ii/SolutionTest.java

use leetcode_in_rust::s0454::p4sum_ii::Solution;

#[test]
fn test_four_sum_count() {
    // nums1=[1,2], nums2=[-2,-1], nums3=[-1,2], nums4=[0,2]
    assert_eq!(
        Solution::four_sum_count(
            vec![1, 2],
            vec![-2, -1],
            vec![-1, 2],
            vec![0, 2]
        ),
        2
    );
}

#[test]
fn test_four_sum_count2() {
    // nums1=[0], nums2=[0], nums3=[0], nums4=[0]
    assert_eq!(
        Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]),
        1
    );
}
