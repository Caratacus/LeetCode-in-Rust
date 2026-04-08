// Problem 3175: find the first player to win k games in a row
// #Medium #Array #Simulation #2024_06_12_Time_1_ms_(100.00%)_Space_60.4_MB_(70.11%)

pub struct Solution;

impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let n = skills.len();
        let mut max = skills[0];
        let mut cnt = 0;
        let mut res = 0;
        for i in 1..n {
            if skills[i] > max {
                max = skills[i];
                cnt = 0;
                res = i as i32;
            }
            cnt += 1;
            if cnt == k {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void findWinningPlayer()
    //   assertThat(new Solution().findWinningPlayer(new int[] {4, 2, 6, 3, 9}, 2), equalTo(2));
    #[test]
    fn test_find_winning_player() {
        assert_eq!(Solution::find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
    }

    // Java: void findWinningPlayer2()
    //   assertThat(new Solution().findWinningPlayer(new int[] {2, 5, 4}, 3), equalTo(1));
    #[test]
    fn test_find_winning_player2() {
        assert_eq!(Solution::find_winning_player(vec![2, 5, 4], 3), 1);
    }
}
