use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck.clone();
        deck.sort();
        let mut queue: VecDeque<usize> = (0..deck.len()).collect();
        let mut res: Vec<i32> = vec![0; deck.len()];

        for n in deck {
            let i = queue.pop_front().unwrap();
            res[i] = n;
            if let Some(j) = queue.pop_front() {
                queue.push_back(j);
            }
        }

        res
    }
}
