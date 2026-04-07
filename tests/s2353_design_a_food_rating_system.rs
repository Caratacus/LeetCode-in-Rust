// Tests for Problem 2353: Design a Food Rating System
// Java reference: src/test/java/g2301_2400/s2353_design_a_food_rating_system/FoodRatingsTest.java

use leetcode_in_rust::s2353::design_a_food_rating_system::FoodRatings;

#[test]
fn test_food_ratings() {
    let mut food_ratings = FoodRatings::new(
        vec![
            "kimchi".to_string(),
            "miso".to_string(),
            "sushi".to_string(),
            "moussaka".to_string(),
            "ramen".to_string(),
            "bulgogi".to_string(),
        ],
        vec![
            "korean".to_string(),
            "japanese".to_string(),
            "japanese".to_string(),
            "greek".to_string(),
            "japanese".to_string(),
            "korean".to_string(),
        ],
        vec![9, 12, 8, 15, 14, 7],
    );
    // return "kimchi"
    // "kimchi" is the highest rated korean food with a rating of 9.
    assert_eq!(
        food_ratings.highest_rated("korean".to_string()),
        "kimchi".to_string()
    );
    // return "ramen"
    // "ramen" is the highest rated japanese food with a rating of 14.
    assert_eq!(
        food_ratings.highest_rated("japanese".to_string()),
        "ramen".to_string()
    );
    // "sushi" now has a rating of 16.
    food_ratings.change_rating("sushi".to_string(), 16);
    // return "sushi"
    // "sushi" is the highest rated japanese food with a rating of 16.
    assert_eq!(
        food_ratings.highest_rated("japanese".to_string()),
        "sushi".to_string()
    );
    // "ramen" now has a rating of 16.
    food_ratings.change_rating("ramen".to_string(), 16);
    // return "ramen"
    // Both "sushi" and "ramen" have a rating of 16.
    // However, "ramen" is lexicographically smaller than "sushi".
    assert_eq!(
        food_ratings.highest_rated("japanese".to_string()),
        "ramen".to_string()
    );
}
