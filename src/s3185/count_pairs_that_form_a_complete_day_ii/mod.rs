// Problem 3185: count pairs that form a complete day ii
// #Medium #Array #Hash_Table #Counting #2024_06_21_Time_3_ms_(97.60%)_Space_97.1_MB_(14.69%)

pub struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut hour = vec![0i64; 24];
        for j in hours {
            hour[(j % 24) as usize] += 1;
        }
        let mut counter: i64 = hour[0] * (hour[0] - 1) / 2;
        counter += hour[12] * (hour[12] - 1) / 2;
        for i in 1..12 {
            counter += hour[i] * hour[24 - i];
        }
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countCompleteDayPairs()
    //   assertThat(
    //   new Solution().countCompleteDayPairs(new int[] {12, 12, 30, 24, 24}), equalTo(2L));
    #[test]
    fn test_count_complete_day_pairs() {
        assert_eq!(
            Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]),
            2
        );
    }

    // Java: void countCompleteDayPairs2()
    //   assertThat(new Solution().countCompleteDayPairs(new int[] {72, 48, 24, 3}), equalTo(3L));
    #[test]
    fn test_count_complete_day_pairs2() {
        assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
    }
}
