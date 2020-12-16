/**
 * @param {string} s
 * @return {boolean}
 */
var canPermutePalindrome = function(s) {
  const obj = {};
  for (var i=0;i<s.length;i++) {
    if (obj[s[i]] == null) {
      obj[s[i]] = 1;
    } else {
      obj[s[i]] += 1;
    }
  }
  
  var isOdd = false;
  for (var val of Object.values(obj)) {
    if (isOdd === false && val % 2 === 1) {
      isOdd = true;
    } else if (val % 2 === 1) {
      return false;
    }
  }
  return true;
};
