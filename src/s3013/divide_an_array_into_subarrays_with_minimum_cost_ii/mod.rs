// Problem 3013: divide an array into subarrays with minimum cost ii
// #Hard #Array #Hash_Table #Heap_Priority_Queue #Sliding_Window

use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let k = (k - 1) as usize;
        let dist = dist as usize;
        let n = nums.len();

        // Comparator: compare by (nums[idx], idx)
        // In Rust BTreeSet, we use a tuple (nums[idx], idx) as the key
        let mut used: BTreeSet<(i32, usize)> = BTreeSet::new();
        let mut unused: BTreeSet<(i32, usize)> = BTreeSet::new();

        let mut sum: i64 = 0;
        let mut answer = i64::MAX;

        // Initialize the first window
        let window_end = std::cmp::min(dist + 1, n - 1);
        for current_window in 1..=window_end {
            sum += nums[current_window] as i64;
            used.insert((nums[current_window], current_window));
        }

        // Move excess elements from used to unused
        while used.len() > k {
            if let Some(&largest) = used.last() {
                used.remove(&largest);
                sum -= largest.0 as i64;
                unused.insert(largest);
            }
        }

        answer = answer.min(sum);

        // Slide the window
        for (current_window, prev_window) in (dist + 2..n).zip(1..) {
            unused.insert((nums[current_window], current_window));

            if used.contains(&(nums[prev_window], prev_window)) {
                sum -= nums[prev_window] as i64;
                used.remove(&(nums[prev_window], prev_window));

                if let Some(&smallest) = unused.first() {
                    unused.remove(&smallest);
                    sum += smallest.0 as i64;
                    used.insert(smallest);
                }
            } else {
                unused.remove(&(nums[prev_window], prev_window));

                if let (Some(&used_last), Some(&unused_first)) = (used.last(), unused.first()) {
                    if used_last.0 > unused_first.0 {
                        used.remove(&used_last);
                        sum -= used_last.0 as i64;
                        unused.insert(used_last);

                        unused.remove(&unused_first);
                        sum += unused_first.0 as i64;
                        used.insert(unused_first);
                    }
                }
            }

            answer = answer.min(sum);
        }

        nums[0] as i64 + answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_cost() {
        assert_eq!(Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
    }

    #[test]
    fn test_minimum_cost2() {
        assert_eq!(Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
    }

    #[test]
    fn test_minimum_cost3() {
        assert_eq!(Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);
    }
}
