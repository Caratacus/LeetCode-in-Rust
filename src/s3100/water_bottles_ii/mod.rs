// Problem 3100: Water Bottles II
// #Medium #Math #Simulation #2024_04_19_Time_0_ms_(100.00%)_Space_40.8_MB_(45.33%)

pub struct Solution;

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut empty_bottles = num_bottles;
        let mut bottle_drinks = num_bottles;
        let mut num_exchange = num_exchange;
        while num_exchange <= empty_bottles {
            bottle_drinks += 1;
            empty_bottles = 1 + (empty_bottles - num_exchange);
            num_exchange += 1;
        }
        bottle_drinks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxBottlesDrunk()
    //   assertThat(new Solution().maxBottlesDrunk(13, 6), equalTo(15));
    #[test]
    fn test_max_bottles_drunk() {
        assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
    }

    // Java: void maxBottlesDrunk2()
    //   assertThat(new Solution().maxBottlesDrunk(10, 3), equalTo(13));
    #[test]
    fn test_max_bottles_drunk2() {
        assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
    }
}
