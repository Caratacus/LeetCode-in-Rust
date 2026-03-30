// Problem 2353: design a food rating system

pub struct FoodRatings {
    foods: Vec<String>,
    cuisines: Vec<String>,
    ratings: Vec<i32>,
}

impl FoodRatings {
    pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        todo!()
    }

    pub fn change_rating(&mut self, food: String, new_rating: i32) -> () {
        todo!()
    }

    pub fn highest_rated(&mut self, cuisine: String) -> String {
        todo!()
    }

    pub fn compare(&mut self, f1: String, f2: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void foodRatings()
    //   FoodRatings foodRatings =
    //   new FoodRatings(
    //   new String[] {"kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"},
    //   new String[] {
    //   "korean", "japanese", "japanese", "greek", "japanese", "korean"
    //   ... (19 more lines)
    #[test]
    fn test_food_ratings() {
        // TODO: 翻译 Java 测试
    }
}
