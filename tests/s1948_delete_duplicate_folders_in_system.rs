// Tests for Problem 1948: Delete Duplicate Folders in System
// Java reference: src/test/java/g1901_2000/s1948_delete_duplicate_folders_in_system/SolutionTest.java

use leetcode_in_rust::s1948::delete_duplicate_folders_in_system::Solution;

#[test]
fn test_delete_duplicate_folder() {
    let mut result = Solution::delete_duplicate_folder(vec![
        vec![String::from("a")],
        vec![String::from("c")],
        vec![String::from("d")],
        vec![String::from("a"), String::from("b")],
        vec![String::from("c"), String::from("b")],
        vec![String::from("d"), String::from("a")],
    ]);
    // Sort for comparison since order may vary
    result.sort();
    let mut expected = vec![
        vec![String::from("d")],
        vec![String::from("d"), String::from("a")],
    ];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_delete_duplicate_folder2() {
    let mut result = Solution::delete_duplicate_folder(vec![
        vec![String::from("a")],
        vec![String::from("c")],
        vec![String::from("a"), String::from("b")],
        vec![String::from("c"), String::from("b")],
        vec![String::from("a"), String::from("b"), String::from("x")],
        vec![String::from("a"), String::from("b"), String::from("x"), String::from("y")],
        vec![String::from("w")],
        vec![String::from("w"), String::from("y")],
    ]);
    result.sort();
    let mut expected = vec![
        vec![String::from("a")],
        vec![String::from("a"), String::from("b")],
        vec![String::from("c")],
        vec![String::from("c"), String::from("b")],
    ];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_delete_duplicate_folder3() {
    let mut result = Solution::delete_duplicate_folder(vec![
        vec![String::from("a"), String::from("b")],
        vec![String::from("c"), String::from("d")],
        vec![String::from("c")],
        vec![String::from("a")],
    ]);
    result.sort();
    let mut expected = vec![
        vec![String::from("a")],
        vec![String::from("a"), String::from("b")],
        vec![String::from("c")],
        vec![String::from("c"), String::from("d")],
    ];
    expected.sort();
    assert_eq!(result, expected);
}
