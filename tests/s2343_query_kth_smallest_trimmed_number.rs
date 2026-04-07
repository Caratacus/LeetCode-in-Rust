// Tests for Problem 2343: Query Kth Smallest Trimmed Number
// Java reference: src/test/java/g2301_2400/s2343_query_kth_smallest_trimmed_number/SolutionTest.java

use leetcode_in_rust::s2343::query_kth_smallest_trimmed_number::Solution;

#[test]
fn test_smallest_trimmed_numbers() {
    assert_eq!(
        Solution::smallest_trimmed_numbers(
            vec![String::from("102"), String::from("473"), String::from("251"), String::from("814")],
            vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]]
        ),
        vec![2, 2, 1, 0]
    );
}

#[test]
fn test_smallest_trimmed_numbers2() {
    assert_eq!(
        Solution::smallest_trimmed_numbers(
            vec![String::from("24"), String::from("37"), String::from("96"), String::from("04")],
            vec![vec![2, 1], vec![2, 2]]
        ),
        vec![3, 0]
    );
}
