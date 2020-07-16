/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
var isMatch = function(s, p) {
  var sLen = s.length, pLen = p.length;
  var sIdx = 0, pIdx = 0;
  var starIdx = -1, sTmpIdx = -1;

  while (sIdx < sLen) {
      // If the pattern caracter = string character
      // or pattern character = '?'
    if (pIdx < pLen && (p.charAt(pIdx) === '?' || p.charAt(pIdx) == s.charAt(sIdx))){
        ++sIdx;
        ++pIdx;
    }
      // If pattern character = '*'
    else if (pIdx < pLen && p.charAt(pIdx) === '*') {
        // Check the situation
        // when '*' matches no characters
        starIdx = pIdx;
        sTmpIdx = sIdx;
        ++pIdx;
    }
      // If pattern character != string character
      // or pattern is used up
      // and there was no '*' character in pattern 
    else if (starIdx === -1) {
        return false;
    }
      // If pattern character != string character
      // or pattern is used up
      // and there was '*' character in pattern before
    else {
        // Backtrack: check the situation
        // when '*' matches one more character
        pIdx = starIdx + 1;
        sIdx = sTmpIdx + 1;
        sTmpIdx = sIdx;
    }
  }

  // The remaining characters in the pattern should all be '*' characters
  for(var i = pIdx; i < pLen; i++)
    if (p.charAt(i) != '*') return false;
  return true;
};
