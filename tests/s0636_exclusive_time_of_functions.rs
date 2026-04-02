// Tests for Problem 0636: Exclusive Time of Functions
// Java reference: src/test/java/g0601_0700/s0636_exclusive_time_of_functions/SolutionTest.java

use leetcode_in_rust::s0636::exclusive_time_of_functions::Solution;

#[test]
fn test_exclusive_time() {
    let logs = vec![
        "0:start:0".to_string(),
        "1:start:2".to_string(),
        "1:end:5".to_string(),
        "0:end:6".to_string(),
    ];
    assert_eq!(Solution::exclusive_time(2, logs), vec![3, 4]);
}

#[test]
fn test_exclusive_time2() {
    let logs = vec![
        "0:start:0".to_string(),
        "0:start:2".to_string(),
        "0:end:5".to_string(),
        "0:start:6".to_string(),
        "0:end:6".to_string(),
        "0:end:7".to_string(),
    ];
    assert_eq!(Solution::exclusive_time(1, logs), vec![8]);
}

#[test]
fn test_exclusive_time3() {
    let logs = vec![
        "0:start:0".to_string(),
        "0:start:2".to_string(),
        "0:end:5".to_string(),
        "1:start:6".to_string(),
        "1:end:6".to_string(),
        "0:end:7".to_string(),
    ];
    assert_eq!(Solution::exclusive_time(2, logs), vec![7, 1]);
}
