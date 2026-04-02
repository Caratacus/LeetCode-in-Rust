// Tests for Problem 0599: Minimum Index Sum of Two Lists
// Java reference: src/test/java/g0501_0600/s0599_minimum_index_sum_of_two_lists/SolutionTest.java

use leetcode_in_rust::s0599::minimum_index_sum_of_two_lists::Solution;

#[test]
fn test_find_restaurant() {
    let list1 = vec!["Shogun".to_string(), "Tapioca Express".to_string(), "Burger King".to_string(), "KFC".to_string()];
    let list2 = vec!["Piatti".to_string(), "The Grill at Torrey Pines".to_string(), "Hungry Hunter Steakhouse".to_string(), "Shogun".to_string()];
    let mut result = Solution::find_restaurant(list1, list2);
    result.sort();
    assert_eq!(result, vec!["Shogun"]);
}

#[test]
fn test_find_restaurant2() {
    let list1 = vec!["Shogun".to_string(), "Tapioca Express".to_string(), "Burger King".to_string(), "KFC".to_string()];
    let list2 = vec!["KFC".to_string(), "Shogun".to_string(), "Burger King".to_string()];
    let mut result = Solution::find_restaurant(list1, list2);
    result.sort();
    assert_eq!(result, vec!["Shogun"]);
}
