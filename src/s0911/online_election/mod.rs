// Problem 0911: online election

pub struct TopVotedCandidate {
    persons: Vec<i32>,
    times: Vec<i32>,
}

impl TopVotedCandidate {
    pub fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        todo!()
    }

    pub fn q(&mut self, t: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void topVotedCandidateTest()
    //   TopVotedCandidate topVotedCandidate =
    //   new TopVotedCandidate(
    //   new int[] {0, 1, 1, 0, 0, 1, 0}, new int[] {0, 5, 10, 15, 20, 25, 30});
    //   assertThat(topVotedCandidate.q(3), equalTo(0));
    //   assertThat(topVotedCandidate.q(12), equalTo(1));
    //   ... (5 more lines)
    #[test]
    fn test_top_voted_candidate_test() {
        // TODO: 翻译 Java 测试
    }
}
