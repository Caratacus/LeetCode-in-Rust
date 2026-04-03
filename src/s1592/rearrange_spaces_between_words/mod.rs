// Problem 1592: Rearrange Spaces Between Words
// #Easy #String
// #Big_O_Time_O(n)_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let space_count = text.chars().filter(|&c| c == ' ').count();

        let words: Vec<&str> = text.split_whitespace().collect();

        if words.len() == 1 {
            let mut result = words[0].to_string();
            for _ in 0..space_count {
                result.push(' ');
            }
            return result;
        }

        let trailing_spaces = space_count % (words.len() - 1);
        let new_spaces = space_count / (words.len() - 1);

        let mut result = String::new();
        for (j, word) in words.iter().enumerate() {
            result.push_str(word);
            if j < words.len() - 1 {
                for _ in 0..new_spaces {
                    result.push(' ');
                }
            } else {
                for _ in 0..trailing_spaces {
                    result.push(' ');
                }
            }
        }
        result
    }
}
