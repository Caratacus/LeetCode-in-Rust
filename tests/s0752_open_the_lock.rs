// Tests for Problem 0752: Open the Lock
// Java reference: src/test/java/g0701_0800/s0752_open_the_lock/SolutionTest.java

use leetcode_in_rust::s0752::open_the_lock::Solution;

#[test]
fn test_open_lock() {
    assert_eq!(
        Solution::open_lock(
            vec![
                "0201".to_string(),
                "0101".to_string(),
                "0102".to_string(),
                "1212".to_string(),
                "2002".to_string()
            ],
            "0202".to_string()
        ),
        6
    );
}

#[test]
fn test_open_lock2() {
    assert_eq!(
        Solution::open_lock(vec!["8888".to_string()], "0009".to_string()),
        1
    );
}

#[test]
fn test_open_lock3() {
    assert_eq!(
        Solution::open_lock(
            vec![
                "8887".to_string(),
                "8889".to_string(),
                "8878".to_string(),
                "8898".to_string(),
                "8788".to_string(),
                "8988".to_string(),
                "7888".to_string(),
                "9888".to_string()
            ],
            "8888".to_string()
        ),
        -1
    );
}
