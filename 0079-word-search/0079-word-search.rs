use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        //early check to make sure the word can actually be built from the board
        if !Solution::check(&board, &word) { return false; }

        let (row, col) = (board.len(), board[0].len());
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for r in 0..row {
            for c in 0..col {
                if Solution::dfs(r, c, 0, &board, &word, &mut visited) {
                    return true
                }
            }
        }

        false
    }

    pub fn dfs(
        r: usize, 
        c: usize,
        i: usize,
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        visited: &mut HashSet<(usize, usize)>
        ) -> bool {
        //found ans?
        if i == word.len() {
            return true;
        }

        //boundary conditions
        if r < 0 || c < 0 || r >= board.len() || c >= board[0].len() 
        || word[i] != board[r][c] || visited.contains(&(r, c)) {
          return false;
        }

        //mark cell as visited
        visited.insert((r, c));

        //impl recursive check
        let res =   Self::dfs(r + 1, c, i + 1, board, word, visited) ||
                    Self::dfs(r - 1, c, i + 1, board, word, visited) ||
                    Self::dfs(r, c + 1, i + 1, board, word, visited) ||
                    Self::dfs(r, c - 1, i + 1, board, word, visited);

        //remove from visited as we are done with the recursive search 
         visited.remove(&(r, c));
         
         return res
        }

    pub fn check(board: &Vec<Vec<char>>, word: &Vec<char>) -> bool {
        //early check to make sure the word can actually be built from the board
        let map = board.into_iter().flatten()
        .fold(HashMap::new(), |mut acc, c | {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        for char in word.iter() {
            if !map.contains_key(&char) {
                return false
            }
        }
        true
    }

}