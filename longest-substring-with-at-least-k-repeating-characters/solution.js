/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
var longestSubstring = function(s, k) {
  if (s.length < k) {
    return 0;
  }
  if (s.length === 1) {
    return 1;
  }
  var max = 0;
  for (var i=0;i<s.length;i++) {
    var start = s[i];
    var charTypes = new Map();
    charTypes.set(start, 1);
    if (s.length - i < max) {
      break;
    }
    for (var j=i+1;j<s.length;j++) {
      var end = s[j];
      if (!charTypes.has(end)) {
        charTypes.set(end, 1);
      } else {
        charTypes.set(end, charTypes.get(end)+1);
      }
      if (Array.from(charTypes.values()).every((x) => x >= k)) {
        max = Math.max(max, j-i+1);
      }
    }
  }
  return max;
};
