/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
var lengthOfLongestSubstringKDistinct = function(s, k) {
  var start = 0;
  var end = 0;
  var len = 0;
  var maxlen = 0;
  var map = new Map();
  while(end<s.length) {
    map.set(s[end], end++);

    if (map.size > k) {
      var min = Infinity;
      var mine = null;
      for (const entry of map.entries()) {
        if (min > entry[1]) {
          mine = entry;
          min = entry[1];
        }
      }
      map.delete(mine[0]);
      start = mine[1]+1;
    }
    len = end - start;
    maxlen = Math.max(len, maxlen);
  }
  return maxlen;
};
