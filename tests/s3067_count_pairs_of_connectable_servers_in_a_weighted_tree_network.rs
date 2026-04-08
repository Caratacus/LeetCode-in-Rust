// Tests for Problem 3067: Count Pairs of Connectable Servers in a Weighted Tree Network
// Java reference: src/test/java/g3001_3100/s3067_count_pairs_of_connectable_servers_in_a_weighted_tree_network/SolutionTest.java

use leetcode_in_rust::s3067::count_pairs_of_connectable_servers_in_a_weighted_tree_network::Solution;

#[test]
fn test_count_pairs_of_connectable_servers() {
    assert_eq!(
        Solution::count_pairs_of_connectable_servers(
            vec![
                vec![0, 1, 1],
                vec![1, 2, 5],
                vec![2, 3, 13],
                vec![3, 4, 9],
                vec![4, 5, 2]
            ],
            1
        ),
        vec![0, 4, 6, 6, 4, 0]
    );
}

#[test]
fn test_count_pairs_of_connectable_servers2() {
    assert_eq!(
        Solution::count_pairs_of_connectable_servers(
            vec![
                vec![0, 6, 3],
                vec![6, 5, 3],
                vec![0, 3, 1],
                vec![3, 2, 7],
                vec![3, 1, 6],
                vec![3, 4, 2]
            ],
            3
        ),
        vec![2, 0, 0, 0, 0, 0, 2]
    );
}
