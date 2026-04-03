// Tests for Problem 0806: Number of Lines To Write String
// Java reference: src/test/java/g0701_0800/s0806_number_of_lines_to_write_string/SolutionTest.java

use leetcode_in_rust::s0806::number_of_lines_to_write_string::Solution;

#[test]
fn test_number_of_lines() {
    assert_eq!(
        Solution::number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10
            ],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ),
        vec![3, 60]
    );
}

#[test]
fn test_number_of_lines2() {
    assert_eq!(
        Solution::number_of_lines(
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10
            ],
            "bbbcccdddaaa".to_string()
        ),
        vec![2, 4]
    );
}
