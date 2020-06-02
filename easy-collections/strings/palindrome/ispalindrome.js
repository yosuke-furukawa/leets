var isPalindrome = function(s) {
  var words = s.toLowerCase().split("").filter((w) => 
    (w.charCodeAt() <= 122 && w.charCodeAt() >= 97) ||
    (w.charCodeAt() <= 57 && w.charCodeAt() >= 48)
  )

  for (var i=0;i<words.length/2; i++) {
    if (words[i] !== words[words.length - i -1]) {
      return false;
    }
  }
    
  return true;
};

console.log(isPalindrome("A man, a plan, a canal: Panama"));
console.log(isPalindrome("race a car"));
console.log(isPalindrome("0P"));
