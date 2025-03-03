function isIsomorphic(s: string, t: string): boolean {
    let s_map: Record<string, string> = {};
    let used_chars: Set<string> = new Set();


    for(let i = 0; i < s.length; i++) {
        if(!s_map[s[i]]) {
            if(used_chars.has(t[i])) return false
            s_map[s[i]] = t[i]
            used_chars.add(t[i])
        }

        if(s_map[s[i]] != t[i]) return false
    }

    return true
};