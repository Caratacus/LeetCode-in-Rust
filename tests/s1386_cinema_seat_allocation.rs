// Tests for Problem 1386: Cinema Seat Allocation
// Java reference: src/test/java/g1301_1400/s1386_cinema_seat_allocation/SolutionTest.java

use leetcode_in_rust::s1386::cinema_seat_allocation::Solution;

#[test]
fn test_max_number_of_families() {
    assert_eq!(
        Solution::max_number_of_families(
            3,
            vec![vec![1, 2], vec![1, 3], vec![1, 8], vec![2, 6], vec![3, 1], vec![3, 10]]
        ),
        4
    );
}

#[test]
fn test_max_number_of_families2() {
    assert_eq!(
        Solution::max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]),
        2
    );
}

#[test]
fn test_max_number_of_families3() {
    assert_eq!(
        Solution::max_number_of_families(4, vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]]),
        4
    );
}
