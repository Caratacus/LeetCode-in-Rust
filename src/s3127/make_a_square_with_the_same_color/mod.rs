// Problem 3127: make a square with the same color
// #Easy #Array #Matrix #Enumeration #2024_05_02_Time_0_ms_(100.00%)_Space_41.7_MB_(64.59%)

pub struct Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        for i in 0..n - 1 {
            for j in 0..m - 1 {
                let mut count_black = 0;
                let mut count_white = 0;
                for k in i..=i + 1 {
                    for l in j..=j + 1 {
                        if grid[k][l] == 'W' {
                            count_white += 1;
                        } else {
                            count_black += 1;
                        }
                    }
                }
                if count_black >= 3 || count_white >= 3 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_make_square() {
        assert_eq!(
            Solution::can_make_square(vec![
                vec!['B', 'W', 'B'],
                vec!['B', 'W', 'W'],
                vec!['B', 'W', 'B']
            ]),
            true
        );
    }

    #[test]
    fn test_can_make_square2() {
        assert_eq!(
            Solution::can_make_square(vec![
                vec!['B', 'W', 'B'],
                vec!['W', 'B', 'W'],
                vec!['B', 'W', 'B']
            ]),
            false
        );
    }

    #[test]
    fn test_can_make_square3() {
        assert_eq!(
            Solution::can_make_square(vec![
                vec!['B', 'W', 'B'],
                vec!['B', 'W', 'W'],
                vec!['B', 'W', 'W']
            ]),
            true
        );
    }
}
