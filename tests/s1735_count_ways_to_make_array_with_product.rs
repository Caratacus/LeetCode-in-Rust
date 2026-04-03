// Tests for Problem 1735: Count Ways to Make Array With Product
// Java reference: src/test/java/g1701_1800/s1735_count_ways_to_make_array_with_product/SolutionTest.java

use leetcode_in_rust::s1735::count_ways_to_make_array_with_product::Solution;

#[test]
fn test_ways_to_fill_array() {
    assert_eq!(
        Solution::ways_to_fill_array(vec![vec![2, 6], vec![5, 1], vec![73, 660]]),
        vec![4, 1, 50734910]
    );
}

#[test]
fn test_ways_to_fill_array2() {
    assert_eq!(
        Solution::ways_to_fill_array(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]),
        vec![1, 2, 3, 10, 5]
    );
}
