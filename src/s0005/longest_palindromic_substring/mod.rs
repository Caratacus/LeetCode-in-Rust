// Problem 0005: longest palindromic substring
// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #String #Dynamic_Programming
// #Data_Structure_II_Day_9_String #Algorithm_II_Day_14_Dynamic_Programming
// #Dynamic_Programming_I_Day_17 #Udemy_Strings #Top_Interview_150_Multidimensional_DP
// #Big_O_Time_O(n)_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        if n == 0 {
            return String::new();
        }

        // Transform string: insert '#' between characters
        let mut new_str: Vec<char> = Vec::with_capacity(2 * n + 1);
        new_str.push('#');
        for &c in &chars {
            new_str.push(c);
            new_str.push('#');
        }

        let new_len = new_str.len();
        let mut dp = vec![0i32; new_len];
        let mut friend_center = 0usize;
        let mut friend_radius = 0i32;
        let mut lps_center = 0usize;
        let mut lps_radius = 0i32;

        for i in 0..new_len {
            dp[i] = if friend_center + friend_radius as usize > i {
                dp[2 * friend_center - i].min((friend_center + friend_radius as usize - i) as i32)
            } else {
                1
            };

            while (i + dp[i] as usize) < new_len
                && i >= dp[i] as usize
                && new_str[i + dp[i] as usize] == new_str[i - dp[i] as usize]
            {
                dp[i] += 1;
            }

            if (friend_center + friend_radius as usize) < (i + dp[i] as usize) {
                friend_center = i;
                friend_radius = dp[i];
            }

            if lps_radius < dp[i] {
                lps_center = i;
                lps_radius = dp[i];
            }
        }

        let start = (lps_center - (lps_radius - 1) as usize) / 2;
        let end = (lps_center + lps_radius as usize - 1) / 2;
        s[start..end].to_string()
    }
}
