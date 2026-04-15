// Problem 3360: Stone Removal Game
// #Easy #Math #Simulation

pub struct Solution;

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        if n < 10 {
            return false;
        }
        let mut stones_remaining = n - 10;
        let mut stones_to_be_removed = 9;
        let mut i = 1;
        while stones_remaining >= stones_to_be_removed {
            stones_remaining -= stones_to_be_removed;
            i += 1;
            stones_to_be_removed -= 1;
        }
        i % 2 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void canAliceWin()
    //   assertThat(new Solution().canAliceWin(12), equalTo(true));
    #[test]
    fn test_can_alice_win() {
        // TODO: 翻译 Java 测试
    }

    // Java: void canAliceWin2()
    //   assertThat(new Solution().canAliceWin(1), equalTo(false));
    #[test]
    fn test_can_alice_win2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void canAliceWin3()
    //   assertThat(new Solution().canAliceWin(19), equalTo(false));
    #[test]
    fn test_can_alice_win3() {
        // TODO: 翻译 Java 测试
    }
}
