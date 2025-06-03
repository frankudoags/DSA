function longestPalindrome(s: string): string {
    if (!s || s.length <= 1) { return s }
    let longestPalindrome = s.substring(0, 1)

    for (let i = 0; i < s.length; i++){
        let longestEvenPalindrome = findPalindrome(s, i, i);
        let longestOddPalindrome = findPalindrome(s, i - 1, i);
        [longestEvenPalindrome, longestOddPalindrome].forEach((palindrome) => {
            if(palindrome.length > longestPalindrome.length) {
                longestPalindrome = palindrome;
            }
        })
    }

    return longestPalindrome
};

    const findPalindrome = (s: string, l: number, r: number): string => {
        while(l >=0 && r <= s.length - 1 && s[l] === s[r]) {
            l--;
            r++;
        }
        return s.substring(l + 1, r);
    }