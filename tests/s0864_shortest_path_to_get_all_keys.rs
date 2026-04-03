// Tests for Problem 0864: Shortest Path to Get All Keys
// Java reference: src/test/java/g0801_0900/s0864_shortest_path_to_get_all_keys/SolutionTest.java

use leetcode_in_rust::s0864::shortest_path_to_get_all_keys::Solution;

#[test]
fn test_shortest_path_all_keys() {
    assert_eq!(
        Solution::shortest_path_all_keys(vec!["@.a.#".to_string(), "###.#".to_string(), "b.A.B".to_string()]),
        8
    );
}

#[test]
fn test_shortest_path_all_keys2() {
    assert_eq!(
        Solution::shortest_path_all_keys(vec!["@..aA".to_string(), "..B#.".to_string(), "....b".to_string()]),
        6
    );
}

#[test]
fn test_shortest_path_all_keys3() {
    assert_eq!(
        Solution::shortest_path_all_keys(vec!["@Aa".to_string()]),
        -1
    );
}
