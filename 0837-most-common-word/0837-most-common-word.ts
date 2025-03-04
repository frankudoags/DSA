function mostCommonWord(paragraph: string, banned: string[]): string {
    let map: Record<string, number> = {};

    paragraph
    .replace(/[^a-zA-Z ]/g, " ")
    .split(" ")
    .filter(w => w != "")
    .forEach(w => {
        let word = w.toLowerCase();
        if(!map[word]) map[word] = 0;
        map[word]++
    });
    
    let ban_set = new Set(banned);

    let res = '';
    let max = 0;

    for(const [key, value] of Object.entries(map)) {
        if (ban_set.has(key)) continue;
        if(value > max)  {
            res = key;
            max = value;
        }
    }
    return res
};