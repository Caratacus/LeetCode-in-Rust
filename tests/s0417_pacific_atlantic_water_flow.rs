// Tests for Problem 0417: Pacific Atlantic Water Flow
// Java reference: src/test/java/g0401_0500/s0417_pacific_atlantic_water_flow/SolutionTest.java

use leetcode_in_rust::s0417::pacific_atlantic_water_flow::Solution;

#[test]
fn test_pacific_atlantic() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let result = Solution::pacific_atlantic(heights);
    // Expected cells that can reach both oceans
    assert!(result.contains(&vec![0, 4]));
    assert!(result.contains(&vec![1, 3]));
    assert!(result.contains(&vec![1, 4]));
    assert!(result.contains(&vec![2, 2]));
    assert!(result.contains(&vec![3, 0]));
    assert!(result.contains(&vec![3, 1]));
    assert!(result.contains(&vec![4, 0]));
}

#[test]
fn test_pacific_atlantic2() {
    let heights = vec![vec![1]];
    let result = Solution::pacific_atlantic(heights);
    assert_eq!(result, vec![vec![0, 0]]);
}
