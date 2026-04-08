// Tests for Problem 3160: Find the Number of Distinct Colors Among the Balls
// Java reference: src/test/java/g3101_3200/s3160_find_the_number_of_distinct_colors_among_the_balls/SolutionTest.java

use leetcode_in_rust::s3160::find_the_number_of_distinct_colors_among_the_balls::Solution;
#[test]
fn test_query_results() {
    assert_eq!(
        Solution::query_results(vec![1, 4], vec![1, 5}, {1, 3}, {3, 4}, {3, 4}}),
        vec![1, 2, 3, 4]
    );
}
#[test]
fn test_query_results2() {
    assert_eq!(
        Solution::query_results(
            vec![vec![0, 1}, vec![1, 10}, vec![0, 10}, vec![0, 3}},
            vec![1, 2, 3, 4]
        ),
        vec![false]
    );
}
