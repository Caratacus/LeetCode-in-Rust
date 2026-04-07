// Tests for Problem 2115: Find All Possible Recipes from Given Supplies
// Java reference: src/test/java/g2101_2200/s2115_find_all_possible_recipes_from_given_supplies/SolutionTest.java

use leetcode_in_rust::s2115::find_all_possible_recipes_from_given_supplies::Solution;

#[test]
fn test_find_all_recipes() {
    let mut result = Solution::find_all_recipes(
        vec!["bread".to_string()],
        vec![vec!["yeast".to_string(), "flour".to_string()]],
        vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()],
    );
    result.sort();
    assert_eq!(result, vec!["bread"]);
}

#[test]
fn test_find_all_recipes2() {
    let mut result = Solution::find_all_recipes(
        vec!["bread".to_string(), "sandwich".to_string()],
        vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
        ],
        vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()],
    );
    result.sort();
    assert_eq!(result, vec!["bread", "sandwich"]);
}

#[test]
fn test_find_all_recipes3() {
    let mut result = Solution::find_all_recipes(
        vec!["bread".to_string(), "sandwich".to_string(), "burger".to_string()],
        vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
            vec!["sandwich".to_string(), "meat".to_string(), "bread".to_string()],
        ],
        vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()],
    );
    result.sort();
    assert_eq!(result, vec!["bread", "burger", "sandwich"]);
}
