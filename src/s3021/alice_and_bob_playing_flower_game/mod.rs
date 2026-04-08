// Problem 3021: Alice and Bob Playing Flower Game
// #Medium #Math

pub struct Solution;

impl Solution {
    pub fn flower_game(n: i64, m: i64) -> i64 {
        ((n + 1) / 2) * (m / 2) + ((m + 1) / 2) * (n / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void flowerGame()
    //   assertThat(new Solution().flowerGame(3, 2), equalTo(3L));
    #[test]
    fn test_flower_game() {
        assert_eq!(Solution::flower_game(3, 2), 3);
    }

    // Java: void flowerGame2()
    //   assertThat(new Solution().flowerGame(1, 1), equalTo(0L));
    #[test]
    fn test_flower_game2() {
        assert_eq!(Solution::flower_game(1, 1), 0);
    }
}
