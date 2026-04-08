// Problem 3187: peaks in array
// #Hard #Array #Segment_Tree #Binary_Indexed_Tree
// #2024_06_21_Time_18_ms_(100.00%)_Space_137.7_MB_(31.34%)

pub struct Solution;

impl Solution {
    pub fn count_of_peaks(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();
        let mut peaks = vec![false; n];

        // Calculate the size needed for binary indexed tree
        let bit_size = if n > 0 {
            (n.next_power_of_two() * 2) + 1
        } else {
            1
        };
        let mut binary_indexed_tree = vec![0i32; bit_size];

        for i in 1..n - 1 {
            if nums[i] > nums[i - 1].max(nums[i + 1]) {
                peaks[i] = true;
                Self::update(&mut binary_indexed_tree, i + 1, 1);
            }
        }

        let mut result = Vec::new();
        for query in queries {
            if query[0] == 1 {
                let left_index = query[1] as usize;
                let right_index = query[2] as usize;
                result.push(Self::compute_range_sum(
                    &binary_indexed_tree,
                    left_index + 2,
                    right_index,
                ));
            } else {
                let index = query[1] as usize;
                let value = query[2];
                nums[index] = value;
                for i in -1i32..=1 {
                    let affected = index as i32 + i;
                    if affected >= 1 && affected <= n as i32 - 2 {
                        let affected = affected as usize;
                        let peak = nums[affected] > nums[affected - 1].max(nums[affected + 1]);
                        if peak != peaks[affected] {
                            if peak {
                                Self::update(&mut binary_indexed_tree, affected + 1, 1);
                            } else {
                                Self::update(&mut binary_indexed_tree, affected + 1, -1);
                            }
                            peaks[affected] = peak;
                        }
                    }
                }
            }
        }
        result
    }

    fn compute_range_sum(binary_indexed_tree: &[i32], begin_index: usize, end_index: usize) -> i32 {
        if begin_index <= end_index {
            Self::query(binary_indexed_tree, end_index)
                - Self::query(binary_indexed_tree, begin_index - 1)
        } else {
            0
        }
    }

    fn query(binary_indexed_tree: &[i32], mut index: usize) -> i32 {
        let mut result = 0;
        while index != 0 {
            result += binary_indexed_tree[index];
            index -= index & index.wrapping_neg();
        }
        result
    }

    fn update(binary_indexed_tree: &mut [i32], mut index: usize, delta: i32) {
        while index < binary_indexed_tree.len() {
            binary_indexed_tree[index] += delta;
            index += index & index.wrapping_neg();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countOfPeaks()
    //   assertThat(
    //   new Solution()
    //   .countOfPeaks(
    //   new int[] {3, 1, 4, 2, 5}, new int[][] {{2, 3, 4}, {1, 0, 4}}),
    //   equalTo(List.of(0)));
    #[test]
    fn test_count_of_peaks() {
        assert_eq!(
            Solution::count_of_peaks(vec![3, 1, 4, 2, 5], vec![vec![2, 3, 4], vec![1, 0, 4]]),
            vec![0]
        );
    }

    // Java: void countOfPeaks2()
    //   assertThat(
    //   new Solution()
    //   .countOfPeaks(
    //   new int[] {4, 1, 4, 2, 1, 5},
    //   new int[][] {{2, 2, 4}, {1, 0, 2}, {1, 0, 4}}),
    //   equalTo(List.of(0, 1)));
    #[test]
    fn test_count_of_peaks2() {
        assert_eq!(
            Solution::count_of_peaks(
                vec![4, 1, 4, 2, 1, 5],
                vec![vec![2, 2, 4], vec![1, 0, 2], vec![1, 0, 4]]
            ),
            vec![0, 1]
        );
    }
}
