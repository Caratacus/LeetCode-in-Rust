// Tests for Problem 2685: Count the Number of Complete Components
// Java reference: src/test/java/g2601_2700/s2685_count_the_number_of_complete_components/SolutionTest.java

use leetcode_in_rust::s2685::count_the_number_of_complete_components::Solution;

#[test]
fn test_count_complete_components() {
    assert_eq!(
        Solution::count_complete_components(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]),
        3
    );
}

#[test]
fn test_count_complete_components2() {
    assert_eq!(
        Solution::count_complete_components(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
        ),
        1
    );
}
