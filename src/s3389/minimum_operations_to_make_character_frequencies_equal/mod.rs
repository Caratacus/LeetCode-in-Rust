// Problem 3389: minimum operations to make character frequencies equal

pub struct Solution;

impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let mut freqarr = [0i32; 26];
        for b in s.bytes() {
            freqarr[(b - b'a') as usize] += 1;
        }
        let mut ctr = 0;
        let mut max = 0;
        for i in 0..26 {
            if freqarr[i] != 0 {
                ctr += 1;
                max = max.max(freqarr[i]);
            }
        }
        if ctr == 0 {
            return 0;
        }
        let mut minops = 20000i32;
        for j in 0..=max {
            let mut if_del = 0i32;
            let mut if_add = 0i32;
            let mut free = 0i32;
            for i in 0..26 {
                if freqarr[i] == 0 {
                    free = 0;
                    continue;
                }
                if freqarr[i] >= j {
                    if_del = if_del.min(if_add) + freqarr[i] - j;
                    free = freqarr[i] - j;
                    if_add = 20000;
                } else {
                    let curr_if_del = if_del.min(if_add) + freqarr[i];
                    let curr_if_add = (if_add + j - freqarr[i])
                        .min(if_del + 0.max(j - freqarr[i] - free));
                    if_add = curr_if_add;
                    if_del = curr_if_del;
                    free = freqarr[i];
                }
            }
            minops = minops.min(if_del.min(if_add));
        }
        minops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void makeStringGood()
    //   assertThat(new Solution().makeStringGood("acab"), equalTo(1));
    #[test]
    fn test_make_string_good() {
        // TODO: 翻译 Java 测试
    }

    // Java: void makeStringGood2()
    //   assertThat(new Solution().makeStringGood("wddw"), equalTo(0));
    #[test]
    fn test_make_string_good2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void makeStringGood3()
    //   assertThat(new Solution().makeStringGood("aaabc"), equalTo(2));
    #[test]
    fn test_make_string_good3() {
        // TODO: 翻译 Java 测试
    }
}
