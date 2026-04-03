// Problem 1574: Shortest Subarray to be Removed to Make Array Sorted
// #Medium #Array #Binary_Search #Two_Pointers #Stack #Monotonic_Stack
// #Big_O_Time_O(n)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = 0;

        while left < n - 1 && arr[left] <= arr[left + 1] {
            left += 1;
        }

        if left == n - 1 {
            return 0;
        }

        let mut right = n - 1;
        while right > left && arr[right] >= arr[right - 1] {
            right -= 1;
        }

        if right == 0 {
            return (n - 1) as i32;
        }

        let mut result = std::cmp::min(n - left - 1, right);
        let mut i = 0;
        let mut j = right;

        while i <= left && j < n {
            if arr[j] >= arr[i] {
                result = result.min(j - i - 1);
                i += 1;
            } else {
                j += 1;
            }
        }

        result as i32
    }
}
