// Problem 0003: longest substring without repeating characters
// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Hash_Table #Sliding_Window
// #Algorithm_I_Day_6_Sliding_Window #Level_2_Day_14_Sliding_Window/Two_Pointer #Udemy_Strings
// #Top_Interview_150_Sliding_Window #Big_O_Time_O(n)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_indices = [-1i32; 256];
        let mut max_len = 0;
        let mut cur_len = 0;
        let mut start = 0;

        for (i, cur) in s.bytes().enumerate() {
            let i = i as i32;
            let last = last_indices[cur as usize];
            if last < start {
                last_indices[cur as usize] = i;
                cur_len += 1;
            } else {
                start = last + 1;
                cur_len = i - start + 1;
                last_indices[cur as usize] = i;
            }
            if cur_len > max_len {
                max_len = cur_len;
            }
        }
        max_len
    }
}
