// Tests for Problem 0609: Find Duplicate File in System
// Java reference: src/test/java/g0601_0700/s0609_find_duplicate_file_in_system/SolutionTest.java

use leetcode_in_rust::s0609::find_duplicate_file_in_system::Solution;

#[test]
fn test_find_duplicate() {
    let result = Solution::find_duplicate(vec![
        "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
        "root/c 3.txt(abcd)".to_string(),
        "root/c/d 4.txt(efgh)".to_string(),
        "root 4.txt(efgh)".to_string(),
    ]);

    // 结果应包含两个分组
    assert_eq!(result.len(), 2);

    // 检查每个分组（顺序可能不同，但内容应匹配）
    let mut sorted_result: Vec<Vec<String>> = result
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect();
    sorted_result.sort_by(|a, b| a.len().cmp(&b.len()));

    assert_eq!(sorted_result[0], vec!["root/a/1.txt", "root/c/3.txt"]);
    assert_eq!(
        sorted_result[1],
        vec!["root/4.txt", "root/a/2.txt", "root/c/d/4.txt"]
    );
}

#[test]
fn test_find_duplicate2() {
    let result = Solution::find_duplicate(vec![
        "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
        "root/c 3.txt(abcd)".to_string(),
        "root/c/d 4.txt(efgh)".to_string(),
    ]);

    // 结果应包含两个分组
    assert_eq!(result.len(), 2);

    // 检查每个分组
    let mut sorted_result: Vec<Vec<String>> = result
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect();
    sorted_result.sort_by(|a, b| a.len().cmp(&b.len()));

    assert_eq!(sorted_result[0], vec!["root/a/1.txt", "root/c/3.txt"]);
    assert_eq!(sorted_result[1], vec!["root/a/2.txt", "root/c/d/4.txt"]);
}
