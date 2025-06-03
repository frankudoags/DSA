function longestPalindrome(s: string): string {
    if (!s || s.length <= 1) { return s }
    let longest = s.substring(0, 1)

    for (let i = 0; i < s.length; i++){
        let even_pal = findPalindrome(s, i, i);
        let odd_pal = findPalindrome(s, i - 1, i);

        [even_pal, odd_pal].forEach((palindrome) => {
            if(palindrome.length > longest.length) {
                longest = palindrome;
            }
        })
    }

    return longest
};

    const findPalindrome = (s: string, l: number, r: number): string => {
        while(l >=0 && r <= s.length - 1 && s[l] === s[r]) {
            l--;
            r++;
        }
        return s.substring(l + 1, r);
    }