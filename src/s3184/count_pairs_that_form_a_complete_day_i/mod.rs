// Problem 3184: count pairs that form a complete day i
// #Easy #Array #Hash_Table #Counting #2024_06_21_Time_1_ms_(98.20%)_Space_42_MB_(83.72%)

pub struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut modular = vec![0i32; 26];
        let mut ans = 0;
        for hour in hours {
            let m = hour % 24;
            ans += modular[24 - m as usize];
            if m == 0 {
                modular[24] += 1;
            } else {
                modular[m as usize] += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countCompleteDayPairs()
    //   assertThat(
    //   new Solution().countCompleteDayPairs(new int[] {12, 12, 30, 24, 24}), equalTo(2));
    #[test]
    fn test_count_complete_day_pairs() {
        assert_eq!(
            Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]),
            2
        );
    }

    // Java: void countCompleteDayPairs2()
    //   assertThat(new Solution().countCompleteDayPairs(new int[] {72, 48, 24, 3}), equalTo(3));
    #[test]
    fn test_count_complete_day_pairs2() {
        assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
    }
}
