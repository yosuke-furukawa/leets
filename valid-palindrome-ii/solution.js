/**
 * @param {string} s
 * @return {boolean}
 */
const checkPalidrome = function(s, start, end){
    while(start < end){
        if(s[start] != s[end]){
            return false;
        }
        start++;
        end--;
    }
    return true;
}


const validPalindrome = function(s) {
    let start = 0;
    let end = s.length - 1;

    while(start < end){
        if(s[start] != s[end]){
             //Check if start, end - 1 || start + 1, end is a palindrome
            return checkPalidrome(s, start, end -1) || checkPalidrome(s, start + 1, end);
        }
        start++;
        end--;
    }
    return true;
};
