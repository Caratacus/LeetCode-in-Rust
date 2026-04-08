// Tests for Problem 3030: Find the Grid of Region Average
// Java reference: src/test/java/g3001_3100/s3030_find_the_grid_of_region_average/SolutionTest.java
use leetcode_in_rust::s3030::find_the_grid_of_region_average::Solution;
#[test]
fn test_result_grid() {
    assert_eq!(
        Solution::result_grid(vec![vec![5, 6, 7, 10], vec![8, 9, 10, 10], vec![11, 12, 13, 10]], 3),
        vec![vec![9, 9, 9, 9], vec![9, 9, 9, 9], vec![9, 9, 9, 9]]
    );
}
#[test]
fn test_result_grid2() {
    assert_eq!(
        Solution::result_grid(
            vec![vec![10, 20, 30], vec![15, 25, 35], vec![20, 30, 40], vec![25, 35, 45]],
            12
        ),
        vec![vec![25, 25, 25], vec![27, 27, 27], vec![27, 27, 27], vec![30, 30, 30]]
    )
}
#[test]
fn test_result_grid3() {
    assert_eq!(
        Solution::result_grid(vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]], 1),
        vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]]
    )
}
