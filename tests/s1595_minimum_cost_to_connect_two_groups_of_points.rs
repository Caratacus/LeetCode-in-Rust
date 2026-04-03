// Tests for Problem 1595: Minimum Cost to Connect Two Groups of Points
// Java reference: src/test/java/g1501_1600/s1595_minimum_cost_to_connect_two_groups_of_points/SolutionTest.java

use leetcode_in_rust::s1595::minimum_cost_to_connect_two_groups_of_points::Solution;

#[test]
fn test_connect_two_groups() {
    assert_eq!(Solution::connect_two_groups(vec![vec![15, 96], vec![36, 2]]), 17);
}

#[test]
fn test_connect_two_groups2() {
    assert_eq!(Solution::connect_two_groups(vec![vec![1, 3, 5], vec![4, 1, 1], vec![1, 5, 3]]), 4);
}

#[test]
fn test_connect_two_groups3() {
    assert_eq!(
        Solution::connect_two_groups(vec![
            vec![2, 5, 1],
            vec![3, 4, 7],
            vec![8, 1, 2],
            vec![6, 2, 4],
            vec![3, 8, 8]
        ]),
        10
    );
}
