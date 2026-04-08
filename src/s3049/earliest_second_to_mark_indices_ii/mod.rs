// Problem 3049: Earliest Second to Mark Indices II
// #Hard #Array #Greedy #Binary_Search #Heap_Priority_Queue
// #Big_O_Time_O(m*log(m)*log(n))_Space_O(n)

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        let m = change_indices.len();
        let n = nums.len();
        if m < n {
            return -1;
        }

        let mut set: HashSet<i32> = HashSet::new();
        let mut first = vec![false; m];

        for i in 0..m {
            if nums[change_indices[i] as usize - 1] > 1 && set.insert(change_indices[i]) {
                first[i] = true;
            }
        }

        let mut sum: i64 = nums.iter().map(|&x| x as i64).sum();
        sum += n as i64;

        let mut l = n;
        let mut r = (sum.min(m as i64) + 1) as usize;

        while l < r {
            let mid = (l + r) / 2;
            if Self::check(mid, &nums, &change_indices, &first, sum) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        if l > sum.min(m as i64) as usize {
            -1
        } else {
            l as i32
        }
    }

    fn check(
        idx: usize,
        nums: &[i32],
        change_indices: &[i32],
        first: &[bool],
        sum: i64,
    ) -> bool {
        let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut need = sum;
        let mut count = 0;

        let mut i = idx as i32 - 1;
        while i >= 0 && need > idx as i64 {
            let i_usize = i as usize;
            if !first[i_usize] {
                count += 1;
                i -= 1;
                continue;
            }
            let num = nums[change_indices[i_usize] as usize - 1];
            pq.push(Reverse(num));
            need -= (num - 1) as i64;

            if pq.len() > count {
                if let Some(Reverse(top)) = pq.pop() {
                    need += (top - 1) as i64;
                    count += 1;
                }
            }
            i -= 1;
        }
        need <= idx as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earliest_second_to_mark_indices() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![3, 2, 3], vec![1, 3, 2, 2, 2, 2, 3]),
            6
        );
    }

    #[test]
    fn test_earliest_second_to_mark_indices2() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(
                vec![0, 0, 1, 2],
                vec![1, 2, 1, 2, 1, 2, 1, 2]
            ),
            7
        );
    }

    #[test]
    fn test_earliest_second_to_mark_indices3() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![1, 2, 3], vec![1, 2, 3]),
            -1
        );
    }
}
