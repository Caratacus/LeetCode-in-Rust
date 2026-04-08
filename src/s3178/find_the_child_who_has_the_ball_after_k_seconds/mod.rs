// Problem 3178: find the child who has the ball after k seconds
// #Easy #Math #Simulation #2024_06_12_Time_0_ms_(100.00%)_Space_40.4_MB_(93.82%)

pub struct Solution;

impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let big_n = 2 * n - 2;
        let x = k % big_n;
        if x < n {
            x
        } else {
            big_n - x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void numberOfChild()
    //   assertThat(new Solution().numberOfChild(3, 5), equalTo(1));
    #[test]
    fn test_number_of_child() {
        assert_eq!(Solution::number_of_child(3, 5), 1);
    }

    // Java: void numberOfChild2()
    //   assertThat(new Solution().numberOfChild(5, 6), equalTo(2));
    #[test]
    fn test_number_of_child2() {
        assert_eq!(Solution::number_of_child(5, 6), 2);
    }

    // Java: void numberOfChild3()
    //   assertThat(new Solution().numberOfChild(4, 2), equalTo(2));
    #[test]
    fn test_number_of_child3() {
        assert_eq!(Solution::number_of_child(4, 2), 2);
    }
}
