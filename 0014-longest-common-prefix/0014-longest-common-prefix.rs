impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // get the length of the shortest string in the array
      let mut n = strs.iter().map(|s| s.len()).min().unwrap();
    // keep reducing the length by comparing any two strings in the strs array
    // and checking for the common prefix, we reduce n until we find it
    
        while strs.windows(2).any(|s| s[0][..n] != s[1][..n]) { n -= 1}
        //return the longest common prefix
        strs[0][..n].into()
    }
}
