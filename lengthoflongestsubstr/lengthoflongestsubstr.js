/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function(s) {
  var ans = 0;
  var map = new Map();
  for (var i=0, j=0;i<s.length;i++) {
    var c = s[i];
    if (map.has(c)) {
      j = Math.max(map.get(c), j);
    }
    ans = Math.max(ans, i-j+1);
    map.set(c, i+1);
  }
  return ans;
};
