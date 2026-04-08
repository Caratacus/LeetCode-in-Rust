// Problem 3169: count days without meetings
// #Medium #Array #Sorting #2024_06_06_Time_11_ms_(99.96%)_Space_113.7_MB_(5.10%)

pub struct Solution;

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut available_days: Vec<(i32, i32)> = vec![(1, days)];
        // Iterate through each meeting
        for meeting in &meetings {
            let start = meeting[0];
            let end = meeting[1];
            let mut new_available_days: Vec<(i32, i32)> = Vec::new();
            // Iterate through available days and split the intervals
            for interval in &available_days {
                if start > interval.1 || end < interval.0 {
                    // No overlap, keep the interval
                    new_available_days.push(*interval);
                } else {
                    // Overlap, split the interval
                    if interval.0 < start {
                        new_available_days.push((interval.0, start - 1));
                    }
                    if interval.1 > end {
                        new_available_days.push((end + 1, interval.1));
                    }
                }
            }
            available_days = new_available_days;
        }
        // Count the remaining available days
        let mut available_days_count = 0;
        for interval in &available_days {
            available_days_count += interval.1 - interval.0 + 1;
        }
        available_days_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countDays()
    //   assertThat(new Solution().countDays(10, new int[][] {{5, 7}, {1, 3}, {9, 10}}), equalTo(2));
    #[test]
    fn test_count_days() {
        assert_eq!(
            Solution::count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]),
            2
        );
    }

    // Java: void countDays2()
    //   assertThat(new Solution().countDays(5, new int[][] {{2, 4}, {1, 3}}), equalTo(1));
    #[test]
    fn test_count_days2() {
        assert_eq!(Solution::count_days(5, vec![vec![2, 4], vec![1, 3]]), 1);
    }

    // Java: void countDays3()
    //   assertThat(new Solution().countDays(6, new int[][] {{1, 6}}), equalTo(0));
    #[test]
    fn test_count_days3() {
        assert_eq!(Solution::count_days(6, vec![vec![1, 6]]), 0);
    }
}
