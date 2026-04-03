// Tests for Problem 0952: Largest Component Size by Common Factor
// Java reference: src/test/java/g0901_1000/s0952_largest_component_size_by_common_factor/SolutionTest.java

use leetcode_in_rust::s0952::largest_component_size_by_common_factor::Solution;

#[test]
fn test_largest_component_size() {
    let result = Solution::largest_component_size(vec![4, 6, 15, 35]);
    assert_eq!(result, 4);
}

#[test]
fn test_largest_component_size2() {
    let result = Solution::largest_component_size(vec![20, 50, 9, 63]);
    assert_eq!(result, 2);
}

#[test]
fn test_largest_component_size3() {
    let result = Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39]);
    assert_eq!(result, 8);
}
