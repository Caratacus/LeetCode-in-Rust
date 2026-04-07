// Tests for Problem 2537: Count the Number of Good Subarrays
// Java reference: src/test/java/g2501_2600/s2537_count_the_number_of_good_subarrays/SolutionTest.java

use leetcode_in_rust::s2537::count_the_number_of_good_subarrays::Solution;

#[test]
fn test_count_good() {
    assert_eq!(Solution::count_good(vec![1, 1, 1, 1, 1], 10), 1);
}
#[test]
fn test_count_good2() {
    assert_eq!(Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
}
