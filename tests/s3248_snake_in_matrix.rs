// Tests for Problem 3248: Snake in Matrix
// Java reference: src/test/java/g3201_3300/s3248_snake_in_matrix/SolutionTest.java

use leetcode_in_rust::s3248::snake_in_matrix::Solution;

#[test]
fn test_final_position_of_snake() {
    assert_eq!(
        Solution::final_position_of_snake(2, vec!["RIGHT".to_string(), "DOWN".to_string()]),
        3
    );
}

#[test]
fn test_final_position_of_snake2() {
    assert_eq!(
        Solution::final_position_of_snake(
            3,
            vec!["DOWN".to_string(), "RIGHT".to_string(), "UP".to_string()]
        ),
        1
    );
}

#[test]
fn test_final_position_of_snake_all_commands() {
    let commands = vec![
        "UP".to_string(),
        "DOWN".to_string(),
        "LEFT".to_string(),
        "RIGHT".to_string(),
    ];
    let result = Solution::final_position_of_snake(3, commands);
    assert_eq!(result, 4);
}

#[test]
fn test_final_position_of_snake_only_up() {
    let commands = vec!["UP".to_string(), "UP".to_string()];
    let result = Solution::final_position_of_snake(3, commands);
    assert_eq!(result, 0);
}

#[test]
fn test_final_position_of_snake_only_down() {
    let commands = vec!["DOWN".to_string(), "DOWN".to_string()];
    let result = Solution::final_position_of_snake(3, commands);
    assert_eq!(result, 6);
}

#[test]
fn test_final_position_of_snake_only_left() {
    let commands = vec!["LEFT".to_string(), "LEFT".to_string()];
    let result = Solution::final_position_of_snake(3, commands);
    assert_eq!(result, 0);
}

#[test]
fn test_final_position_of_snake_only_right() {
    let commands = vec!["RIGHT".to_string(), "RIGHT".to_string()];
    let result = Solution::final_position_of_snake(3, commands);
    assert_eq!(result, 2);
}

#[test]
fn test_final_position_of_snake_empty_commands() {
    let commands: Vec<String> = vec![];
    let result = Solution::final_position_of_snake(3, commands);
    assert_eq!(result, 0);
}

#[test]
fn test_final_position_of_snake_mixed_commands() {
    let commands = vec![
        "DOWN".to_string(),
        "RIGHT".to_string(),
        "UP".to_string(),
        "LEFT".to_string(),
        "UP".to_string(),
        "DOWN".to_string(),
        "RIGHT".to_string(),
    ];
    let result = Solution::final_position_of_snake(3, commands);
    assert_eq!(result, 4);
}

#[test]
fn test_final_position_of_snake_invalid_commands() {
    let commands = vec![
        "DOWN".to_string(),
        "RIGHT".to_string(),
        "JUMP".to_string(),
        "LEFT".to_string(),
        "UP".to_string(),
        "DOWN".to_string(),
        "RIGHT".to_string(),
    ];
    let result = Solution::final_position_of_snake(3, commands);
    assert_eq!(result, 4);
}
