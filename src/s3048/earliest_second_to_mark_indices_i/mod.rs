// Problem 3048: Earliest Second to Mark Indices I
// #Medium #Array #Binary_Search
// #Big_O_Time_O(m*log(m))_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 || change_indices.is_empty() {
            return 0;
        }

        let change_indices: Vec<i32> = change_indices.iter().map(|&x| x - 1).collect();
        let mut last = vec![-1i32; n];

        let mut low = 0;
        let mut high = change_indices.len() - 1;

        while low < high {
            let mid = low + (high - low) / 2;
            if Self::is_possible(mid, &nums, &change_indices, &mut last) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        if Self::is_possible(low, &nums, &change_indices, &mut last) {
            (low + 1) as i32
        } else {
            -1
        }
    }

    fn is_possible(
        s: usize,
        nums: &[i32],
        change_indices: &[i32],
        last: &mut Vec<i32>,
    ) -> bool {
        let n = nums.len();
        for i in 0..n {
            last[i] = -1;
        }
        for i in 0..=s {
            last[change_indices[i] as usize] = i as i32;
        }

        let mut marked = 0;
        let mut operations = 0i64;

        for i in 0..=s {
            let idx = change_indices[i] as usize;
            if i as i32 == last[idx] {
                if nums[idx] as i64 > operations {
                    return false;
                }
                operations -= nums[idx] as i64;
                marked += 1;
            } else {
                operations += 1;
            }
        }
        marked == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earliest_second_to_mark_indices() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![2, 2, 0], vec![2, 2, 2, 2, 3, 2, 2, 1]),
            8
        );
    }

    #[test]
    fn test_earliest_second_to_mark_indices2() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![1, 3], vec![1, 1, 1, 2, 1, 1, 1]),
            6
        );
    }

    #[test]
    fn test_earliest_second_to_mark_indices3() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![0, 1], vec![2, 2, 2]),
            -1
        );
    }
}
