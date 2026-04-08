// Problem 3092: most frequent ids
// #Medium #Array #Hash_Table #Heap_Priority_Queue #Ordered_Set
// #2024_04_18_Time_3_ms_(100.00%)_Space_69_MB_(49.39%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn most_frequent_i_ds(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
        let max = *nums.iter().max().unwrap_or(&0);
        let n = nums.len();
        let mut bins: HashMap<i32, i64> = HashMap::new();
        let mut most_frequent_id = 0;
        let mut max_count: i64 = 0;
        let mut ans: Vec<i64> = vec![0; n];

        for i in 0..n {
            let count = bins.entry(nums[i]).or_insert(0);
            *count += freq[i] as i64;

            if freq[i] > 0 {
                if *count > max_count {
                    max_count = *count;
                    most_frequent_id = nums[i];
                }
            } else {
                if nums[i] == most_frequent_id {
                    max_count = *bins.get(&nums[i]).unwrap_or(&0);
                    for (&key, &value) in bins.iter() {
                        if value > max_count {
                            max_count = value;
                            most_frequent_id = key;
                        }
                    }
                }
            }
            ans[i] = max_count;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void mostFrequentIDs()
    //   assertThat(
    //   new Solution().mostFrequentIDs(new int[] {2, 3, 2, 1}, new int[] {3, 2, -3, 1}),
    //   equalTo(new long[] {3, 3, 2, 2}));
    #[test]
    fn test_most_frequent_i_ds() {
        assert_eq!(
            Solution::most_frequent_i_ds(vec![2, 3, 2, 1], vec![3, 2, -3, 1]),
            vec![3, 3, 2, 2]
        );
    }

    // Java: void mostFrequentIDs2()
    //   assertThat(
    //   new Solution().mostFrequentIDs(new int[] {5, 5, 3}, new int[] {2, -2, 1}),
    //   equalTo(new long[] {2, 0, 1}));
    #[test]
    fn test_most_frequent_i_ds2() {
        assert_eq!(
            Solution::most_frequent_i_ds(vec![5, 5, 3], vec![2, -2, 1]),
            vec![2, 0, 1]
        );
    }
}
