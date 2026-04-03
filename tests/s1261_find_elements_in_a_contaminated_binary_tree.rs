// Tests for Problem 1261: Find Elements in a Contaminated Binary Tree
// Java reference: src/test/java/g1201_1300/s1261_find_elements_in_a_contaminated_binary_tree/SolutionTest.java

use leetcode_in_rust::s1261::find_elements_in_a_contaminated_binary_tree::FindElements;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_find_elements_test() {
    let root = tree_from_vec(vec![Some(-1), None, Some(-1)]);
    let find_elements = FindElements::new(root);
    assert_eq!(find_elements.find(1), false);
    assert_eq!(find_elements.find(2), true);
}

#[test]
fn test_find_elements_test2() {
    let root = tree_from_vec(vec![Some(-1), Some(-1), Some(-1), Some(-1), Some(-1)]);
    let find_elements = FindElements::new(root);
    assert_eq!(find_elements.find(1), true);
    assert_eq!(find_elements.find(3), true);
    assert_eq!(find_elements.find(5), false);
}

#[test]
fn test_find_elements_test3() {
    let root = tree_from_vec(vec![Some(-1), None, Some(-1), Some(-1), None, Some(-1)]);
    let find_elements = FindElements::new(root);
    assert_eq!(find_elements.find(2), true);
    assert_eq!(find_elements.find(3), false);
    assert_eq!(find_elements.find(4), false);
    assert_eq!(find_elements.find(5), true);
}
