/**
 * @param {string} s
 * @return {number}
 */
var longestPalindrome = function(s) {
  var count = 0;
  var solo = 0;
  s = [...s].sort().join("");
  for (var i=0;i<s.length;i++) {
    var c1 = s[i];
    var c2 = s[i+1];
    if (solo === 0 && c1 !== c2) {
      solo = 1;
    }
    if (c1 === c2) {
      count += 2;
      i = i+1;
    }
  }
  return solo + count;
};
