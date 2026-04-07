// Tests for Problem 2596: Check Knight Tour Configuration
// Java reference: src/test/java/g2501_2600/s2596_check_knight_tour_configuration/SolutionTest.java

use leetcode_in_rust::s2596::check_knight_tour_configuration::Solution;

#[test]
fn test_check_valid_grid() {
    assert_eq!(
        Solution::check_valid_grid(vec![
            vec![0, 11, 16, 5, 20],
            vec![17, 4, 19, 10, 15],
            vec![12, 1, 8, 21, 6],
            vec![3, 18, 23, 14, 9],
            vec![24, 13, 2, 7, 22]
        ]),
        true
    );
}

#[test]
fn test_check_valid_grid2() {
    assert_eq!(
        Solution::check_valid_grid(vec![vec![0, 3, 6], vec![5, 8, 1], vec![2, 7, 4]]),
        false
    );
}
