// Problem 1912: design movie rental system

pub struct MovieRentingSystem {
    n: i32,
    entries: Vec<Vec<i32>>,
}

impl MovieRentingSystem {
    pub fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        todo!()
    }

    pub fn search(&self, movie: i32) -> Vec<i32> {
        todo!()
    }

    pub fn rent(&mut self, shop: i32, movie: i32) -> () {
        todo!()
    }

    pub fn drop(&mut self, shop: i32, movie: i32) -> () {
        todo!()
    }

    pub fn report(&mut self) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void movieRentingSystemTest()
    //   MovieRentingSystem movieRentingSystem =
    //   new MovieRentingSystem(
    //   3,
    //   new int[][] {
    //   {0, 1, 5}, {0, 2, 6}, {0, 3, 7}, {1, 1, 4}, {1, 2, 7}, {2, 1, 5}
    //   ... (9 more lines)
    #[test]
    fn test_movie_renting_system_test() {
        // TODO: 翻译 Java 测试
    }
}
