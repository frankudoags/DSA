use std::collections::HashMap;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        //actual chars array
        let m = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();
        //cipher array
        let mut m_ = Vec::<char>::new();
        //build cipher(strip space, and insert without dupes into cipher arr)
        key.split_whitespace()
            .collect::<String>()
            .chars()
            .for_each(|x| {
                if !m_.contains(&x) {
                    m_.push(x);
                }
            });
        //create a hashmap, keys should be cipher, values are cipher actual char
        // we do this by zipping the cipher and the actual arrays into a hashmap
        let map = m_.iter().zip(m).collect::<HashMap<_, _>>();

        message
            .chars()
            .map(|code| match map.get(&code) {
                Some(decoded) => *decoded,
                None => ' ', //must be space since it was not in the map
            })
            .collect::<String>()
          
    }
}