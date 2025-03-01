function buddyStrings(s: string, goal: string): boolean {
    if(s.length != goal.length) return false;

    let s_char: Record<string, number> = {};
    let g_char: Record<string, number> = {};

    let[diff, dupeCharExists] = [0, false];

    for (let i = 0; i < s.length; i++){
        if(s[i] != goal[i]) diff += 1;
        if(diff > 2) return false

        if(!s_char[s[i]]) s_char[s[i]] = 0;
        if(!g_char[goal[i]]) g_char[goal[i]] = 0;

        s_char[s[i]] += 1;
        g_char[goal[i]] += 1;

        if(s_char[s[i]] > 1) dupeCharExists = true;
    }

    for(const [key] of Object.entries(s_char)) {
        if (s_char[key] != g_char[key]) return false
    }

    //handle s="aa", goal="aa"
    if(s == goal && dupeCharExists) return true

    return diff == 2
};


/**
    Idea is to keep track of if you can swap two letters
    But it wasn't that simple, any more than 2 swap fails,
    if 2 swaps exist, but s can't make goal, still fail(this is why we need a hashmap to track chars)
    finally, if s == goal, you may not find any swaps, but beause of the case where dupes may exist
    like s = "aa" goal = "aa", its true to swap so return true


    finally, you have exhausted all cases, if diff is 2 swaps, return true
 */