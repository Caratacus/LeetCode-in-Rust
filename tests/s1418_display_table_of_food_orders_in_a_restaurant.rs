// Tests for Problem 1418: Display Table of Food Orders in a Restaurant
// Java reference: src/test/java/g1401_1500/s1418_display_table_of_food_orders_in_a_restaurant/SolutionTest.java

use leetcode_in_rust::s1418::display_table_of_food_orders_in_a_restaurant::Solution;

#[test]
fn test_display_table() {
    let orders = vec![
        vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
        vec!["Corina".to_string(), "10".to_string(), "Beef Burrito".to_string()],
        vec!["David".to_string(), "3".to_string(), "Fried Chicken".to_string()],
        vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
        vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
        vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()],
    ];
    let expected = vec![
        vec!["Table".to_string(), "Beef Burrito".to_string(), "Ceviche".to_string(), "Fried Chicken".to_string(), "Water".to_string()],
        vec!["3".to_string(), "0".to_string(), "2".to_string(), "1".to_string(), "0".to_string()],
        vec!["5".to_string(), "0".to_string(), "1".to_string(), "0".to_string(), "1".to_string()],
        vec!["10".to_string(), "1".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
    ];
    assert_eq!(Solution::display_table(orders), expected);
}

#[test]
fn test_display_table2() {
    let orders = vec![
        vec!["James".to_string(), "12".to_string(), "Fried Chicken".to_string()],
        vec!["Ratesh".to_string(), "12".to_string(), "Fried Chicken".to_string()],
        vec!["Amadeus".to_string(), "12".to_string(), "Fried Chicken".to_string()],
        vec!["Adam".to_string(), "1".to_string(), "Canadian Waffles".to_string()],
        vec!["Brianna".to_string(), "1".to_string(), "Canadian Waffles".to_string()],
    ];
    let expected = vec![
        vec!["Table".to_string(), "Canadian Waffles".to_string(), "Fried Chicken".to_string()],
        vec!["1".to_string(), "2".to_string(), "0".to_string()],
        vec!["12".to_string(), "0".to_string(), "3".to_string()],
    ];
    assert_eq!(Solution::display_table(orders), expected);
}

#[test]
fn test_display_table3() {
    let orders = vec![
        vec!["Laura".to_string(), "2".to_string(), "Bean Burrito".to_string()],
        vec!["Jhon".to_string(), "2".to_string(), "Beef Burrito".to_string()],
        vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()],
    ];
    let expected = vec![
        vec!["Table".to_string(), "Bean Burrito".to_string(), "Beef Burrito".to_string(), "Soda".to_string()],
        vec!["2".to_string(), "1".to_string(), "1".to_string(), "1".to_string()],
    ];
    assert_eq!(Solution::display_table(orders), expected);
}
