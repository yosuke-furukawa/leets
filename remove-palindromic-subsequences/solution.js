/**
 * @param {string} s
 * @return {number}
 */
var removePalindromeSub = function(s) {
  if (s.length === 0) {
    return 0;
  }
  var half = s.length / 2;
  for (var i=0;i<half;i++) {
    if (s[i] !== s[s.length - i - 1]) {
      return 2;
    }
  }
  return 1;
};
