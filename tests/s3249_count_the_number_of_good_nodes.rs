// Tests for Problem 3249: Count the Number of Good Nodes
// Java reference: src/test/java/g3201_3300/s3249_count_the_number_of_good_nodes/SolutionTest.java

use leetcode_in_rust::s3249::count_the_number_of_good_nodes::Solution;

#[test]
fn test_count_good_nodes() {
    assert_eq!(
        Solution::count_good_nodes(vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6]
        ]),
        7
    );
}

#[test]
fn test_count_good_nodes2() {
    assert_eq!(
        Solution::count_good_nodes(vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![0, 5],
            vec![1, 6],
            vec![2, 7],
            vec![3, 8]
        ]),
        6
    );
}
