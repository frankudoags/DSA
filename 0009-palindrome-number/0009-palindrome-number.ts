function isPalindrome(x: number): boolean {
    if(x < 0) return false;
    if(x < 10) return true;
    const str = x.toString();
    const strRev = str.split('').reverse().join('');
    return str === strRev;
};