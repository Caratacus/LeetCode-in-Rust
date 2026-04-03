// Tests for Problem 1606: Find Servers That Handled Most Number of Requests
// Java reference: src/test/java/g1601_1700/s1606_find_servers_that_handled_most_number_of_requests/SolutionTest.java

use leetcode_in_rust::s1606::find_servers_that_handled_most_number_of_requests::Solution;

#[test]
fn test_busiest_servers() {
    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]),
        vec![1]
    );
}

#[test]
fn test_busiest_servers2() {
    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]),
        vec![0]
    );
}

#[test]
fn test_busiest_servers3() {
    assert_eq!(
        Solution::busiest_servers(3, vec![1, 2, 3], vec![10, 12, 11]),
        vec![0, 1, 2]
    );
}
