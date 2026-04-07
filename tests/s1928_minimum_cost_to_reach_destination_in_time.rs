// Tests for Problem 1928: Minimum Cost to Reach Destination in Time
// Java reference: src/test/java/g1901_2000/s1928_minimum_cost_to_reach_destination_in_time/SolutionTest.java

use leetcode_in_rust::s1928::minimum_cost_to_reach_destination_in_time::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(
        Solution::min_cost(
            30,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15]
            ],
            vec![5, 1, 2, 20, 20, 3]
        ),
        11
    );
}

#[test]
fn test_min_cost2() {
    assert_eq!(
        Solution::min_cost(
            29,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15]
            ],
            vec![5, 1, 2, 20, 20, 3]
        ),
        48
    );
}

#[test]
fn test_min_cost3() {
    assert_eq!(
        Solution::min_cost(
            25,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15]
            ],
            vec![5, 1, 2, 20, 20, 3]
        ),
        -1
    );
}
