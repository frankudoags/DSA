use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());

       let mut map: HashMap<u8,u8> = HashMap::with_capacity(s.len());

       for i in 0..s.len() {
            let val = map.entry(s[i]).or_insert(t[i]);
            if *val != t[i] {
                return false
            }
       }

    //no of keys must match no of values
       map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()

    }
}