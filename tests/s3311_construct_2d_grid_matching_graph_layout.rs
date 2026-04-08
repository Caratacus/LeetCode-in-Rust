// Tests for Problem 3311: Construct 2D Grid Matching Graph Layout
// Java reference: src/test/java/g3301_3400/s3311_construct_2d_grid_matching_graph_layout/SolutionTest.java

use leetcode_in_rust::s3311::construct_2d_grid_matching_graph_layout::Solution;

#[test]
fn test_construct_grid_layout() {
    assert_eq!(
        Solution::construct_grid_layout(4, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]),
        vec![vec![0, 2], vec![1, 3]]
    );
}

#[test]
fn test_construct_grid_layout2() {
    assert_eq!(
        Solution::construct_grid_layout(5, vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![2, 4]]),
        vec![vec![0, 1, 3, 2, 4]]
    );
}

#[test]
fn test_construct_grid_layout3() {
    assert_eq!(
        Solution::construct_grid_layout(
            9,
            vec![
                vec![0, 1], vec![0, 4], vec![0, 5], vec![1, 7], vec![2, 3], vec![2, 4],
                vec![2, 5], vec![3, 6], vec![4, 6], vec![4, 7], vec![6, 8], vec![7, 8]
            ]
        ),
        vec![vec![1, 0, 5], vec![7, 4, 2], vec![8, 6, 3]]
    );
}
