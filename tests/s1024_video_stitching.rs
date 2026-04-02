// Tests for Problem 1024: Video Stitching
// Java reference: src/test/java/g1001_1100/s1024_video_stitching/SolutionTest.java

use leetcode_in_rust::s1024::video_stitching::Solution;

#[test]
fn test_video_stitching() {
    assert_eq!(
        Solution::video_stitching(
            vec![vec![0, 2], vec![4, 6], vec![8, 10], vec![1, 9], vec![1, 5], vec![5, 9]],
            10
        ),
        3
    );
}

#[test]
fn test_video_stitching2() {
    assert_eq!(
        Solution::video_stitching(
            vec![
                vec![0, 1],
                vec![6, 8],
                vec![0, 2],
                vec![5, 6],
                vec![0, 4],
                vec![0, 3],
                vec![6, 7],
                vec![1, 3],
                vec![4, 7],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
                vec![3, 4],
                vec![4, 5],
                vec![5, 7],
                vec![6, 9]
            ],
            9
        ),
        3
    );
}

#[test]
fn test_video_stitching3() {
    assert_eq!(Solution::video_stitching(vec![vec![0, 1], vec![1, 2]], 5), -1);
}
